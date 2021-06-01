pipeline {
  podTemplate(containers: [
    containerTemplate(name: 'agent', image: 'jenkins/agent:latest', ttyEnabled: true, command: 'sleep')
  ]) {
    node(POD_LABEL) {
      stage('Build') {
        sh 'docker build -t baebot .'
      }
    }
  }
}
