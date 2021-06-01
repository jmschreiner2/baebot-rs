podTemplate(yaml: """
apiVersion: v1
kind: Pod
spec:
  serviceAccountName: jenkins
  automountServiceAccountToken: true
  containers:
  - name: docker
    image: docker
    command: ['sleep']
    args: ['99d']
    tty: true
    volumeMounts:
    - name: dockersock
      mountPath: /var/run/docker.sock
  - name: kubectl
    image: bitnami/kubectl
    command: ['sleep']
    args: ['99d']
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
        withCredentials([usernamePassword(credentialsId: 'dockerhub', usernameVariable: 'USERNAME', passwordVariable: 'PASSWORD')]) {
          sh "docker login -u $USERNAME -p $PASSWORD"
          sh "docker push jmschreiner/baebot:latest"
          sh "docker push jmschreiner/baebot:$BUILD_NUMBER"
        }
      }
    }

    stage('Deploy') {
      container('kubectl') {
        contentReplace(
          configs: [
            variablesReplaceConfig(
              configs: [
                variablesReplaceItemConfig(
                  name: 'TAG',
                  value: '$BUILD_NUMBER'
                )
              ],
              fileEncoding: 'UTF-8',
              filePath: 'k8s/20-deployment.yml',
              variablesPrefix: '#{',
              variablesSuffix: '}#'
            )
          ]
        )
        sh "ls ./k8s | sort | xargs kubectl apply -f"
      }
    }
  }
}
