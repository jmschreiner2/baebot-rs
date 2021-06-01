podTemplate(containers: [
  containerTemplate(name: 'docker', image: 'docker', ttyEnabled: true)
]) {
  node(POD_LABEL) {
    stage('Build') {
      container('agent') {
        sh "echo 'hello from $POD_CONTAINER'"
        sh "docker build -t baebot ."
      }
    }
  }
}
