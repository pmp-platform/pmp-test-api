# pmp-test-api Helm Chart

This Helm chart deploys the pmp-test-api application to Kubernetes.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.0+

## Installation

### Add the dependency repository

```bash
helm repo add pmp-helm-charts https://comfortablynumb.github.io/pmp-helm-charts
helm repo update
```

### Install dependencies

```bash
cd helm/pmp-test-api
helm dependency update
```

### Install the chart

```bash
helm install pmp-test-api . -n <namespace>
```

Or with custom values:

```bash
helm install pmp-test-api . -n <namespace> -f custom-values.yaml
```

## Uninstallation

```bash
helm uninstall pmp-test-api -n <namespace>
```

## Configuration

The chart depends on the `application` chart from the pmp-helm-charts repository. All configuration values should be placed under the `application` key in `values.yaml`.

See `values.yaml` for the full list of available configuration options.

## Upgrading

```bash
helm upgrade pmp-test-api . -n <namespace>
```
