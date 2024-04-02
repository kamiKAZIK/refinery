{{/* vim: set filetype=mustache: */}}
{{/*
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
*/}}

{{- define "refinery.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "refinery.name" -}}
{{- default .Chart.Name .Values.refinery.nameOverride | replace "+" "_" | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "refinery.fullname" -}}
{{- $name := default .Chart.Name .Values.refinery.nameOverride -}}
{{- if contains $name .Release.Name -}}
{{- .Release.Name | replace "+" "_" | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- printf "%s-%s" $name .Release.Name | replace "+" "_" | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}

{{- define "refinery.labels.standard" -}}
app.kubernetes.io/name: {{ include "refinery.name" . }}
helm.sh/chart: {{ include "refinery.chart" . }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/version: {{ required ".Values.refinery.domain.server.imageTag is required!" .Values.refinery.domain.server.imageTag | quote }}
{{- end -}}

{{- define "refinery.labels.matchLabels" -}}
app.kubernetes.io/name: {{ include "refinery.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end -}}

{{- define "refinery.capabilities.kubeVersion" -}}
{{- .Capabilities.KubeVersion.Version -}}
{{- end -}}

{{- define "refinery.capabilities.ingress.apiVersion" -}}
{{- if semverCompare "<1.14-0" (include "refinery.capabilities.kubeVersion" .) -}}
{{- print "extensions/v1beta1" -}}
{{- else if semverCompare "<1.19-0" (include "refinery.capabilities.kubeVersion" .) -}}
{{- print "networking.k8s.io/v1beta1" -}}
{{- else -}}
{{- print "networking.k8s.io/v1" -}}
{{- end -}}
{{- end -}}

{{- define "refinery.capabilities.podDisruptionBudget.apiVersion" -}}
{{- if semverCompare "<1.21-0" (include "refinery.capabilities.kubeVersion" .) -}}
{{- print "policy/v1beta1" -}}
{{- else -}}
{{- print "policy/v1" -}}
{{- end -}}
{{- end -}}

{{- define "refinery.ingress.backend" -}}
{{- $apiVersion := (include "refinery.capabilities.ingress.apiVersion" .context) -}}
{{- if or (eq $apiVersion "extensions/v1beta1") (eq $apiVersion "networking.k8s.io/v1beta1") -}}
serviceName: {{ .serviceName }}
servicePort: {{ .servicePort }}
{{- else -}}
service:
  name: {{ .serviceName }}
  port:
    {{- if typeIs "string" .servicePort }}
    name: {{ .servicePort }}
    {{- else if or (typeIs "int" .servicePort) (typeIs "float64" .servicePort) }}
    number: {{ .servicePort | int }}
    {{- end }}
{{- end -}}
{{- end -}}

{{- define "refinery.ingress.supportsPathType" -}}
{{- if semverCompare "<1.18-0" (include "refinery.capabilities.kubeVersion" .) -}}
{{- print "false" -}}
{{- else -}}
{{- print "true" -}}
{{- end -}}
{{- end -}}
