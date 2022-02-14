pipeline {
    agent {
        node {
            label 'rust-agent'
        }
    }
    stages{
        stage('Build rust project') {
            steps {
                script {
                    sh """
                        /home/jenkins/.cargo/bin/cargo build
                    """
                }
            }
        }
        stage('Build and push') {
            steps {
                script {
                    def app = docker.build("devopsuae/rs-random-number")
                    docker.withRegistry("https://registry.hub.docker.com", "dockerhub") {
                        app.push("${env.BUILD_NUMBER}")
                        app.push("latest")
                    }
                }
            }
        }
    }
}