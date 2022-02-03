pipeline {
    agent {
        node {
            label 'rust-agent'
        }
    }
    stages{
        stage('Say hi') {
            steps {
                script {
                    sh """
                        echo "Hello world"
                    """
                }
            }
        }
    }
}