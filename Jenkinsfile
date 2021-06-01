podTemplate(containers: [
  containerTemplate(name: 'agent', image: 'jenkins/agent:latest', ttyEnabled: true, command: 'sleep')
]) {
  node(POD_LABEL) {
    stage('Build') {
      container('agent') {
        sh "echo hellow from $POD_CONTAINER"
        sh "docker build -t baebot ."
      }
    }
  }
}
