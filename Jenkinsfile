podTemplate(yaml: """
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: docker
    image: docker
    command: ['sleep']
    args: ['99d']
    tty: true
    volumeMounts:
    - name: dockersock
      mountPath: /var/run/docker.sock
  volumes:
  - name: dockersock
    hostPath:
      path: /var/run/docker/sock
""") {
  node(POD_LABEL) {
    stage('Build') {
      container('agent') {
        sh "docker build -t baebot ."
      }
    }
  }
}
