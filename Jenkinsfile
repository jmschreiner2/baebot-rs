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
      path: /var/run/docker.sock
""") {
  node(POD_LABEL) {
    stage('Build') {
      checkout scm
      container('docker') {
        sh "docker build -t baebot ."
      }
    }

    stage('Tag') {
      container('docker') {
        sh "docker tag baebot jmschreiner/baebot:latest"
        sh "docker tag baebot jmschreiner/baebot:$BUILD_NUMBER"
      }
    }

    stage('Publish') {
      container('docker') {
        withCredentials([string(credentialsId: 'dockerhub', variable: 'USERPASS')]) {
        sh "docker login -u 'jmschreiner' -p '<$USERPASS>'"
        sh "docker push jmschreiner/baebot:latest"
        sh "docker push jmschreiner/baebot:$BUILD_NUMBER"
      }
    }
  }
}
