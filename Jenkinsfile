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
        stage('Run rust project') {
            steps {
                script {
                    sh """
                        /home/jenkins/.cargo/bin/cargo run
                    """
                }
            }
        }
    }
}