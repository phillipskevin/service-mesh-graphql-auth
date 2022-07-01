FROM envoyproxy/envoy:v1.20-latest
EXPOSE 80
ENTRYPOINT /usr/local/bin/envoy -c /etc/envoy.yaml -l debug --service-cluster proxy 
