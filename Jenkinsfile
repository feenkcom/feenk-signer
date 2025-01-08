import hudson.tasks.test.AbstractTestResultAction
import hudson.model.Actionable
import hudson.tasks.junit.CaseResult

pipeline {
    agent none
    parameters {
        choice(name: 'BUMP', choices: ['minor', 'patch', 'major'], description: 'What to bump when releasing') }
    options {
        buildDiscarder(logRotator(numToKeepStr: '50'))
        disableConcurrentBuilds()
    }
    environment {
        GITHUB_TOKEN = credentials('githubrelease')

        TOOL_NAME = 'feenk-signer'

        MACOS_INTEL_TARGET = 'x86_64-apple-darwin'
        MACOS_M1_TARGET = 'aarch64-apple-darwin'
    }

    stages {
        stage ('Read tool versions') {
            agent {
                label "${MACOS_M1_TARGET}"
            }
            steps {
                script {
                    FEENK_RELEASER_VERSION = sh (
                            script: "cat feenk-releaser.version",
                            returnStdout: true
                    ).trim()
                }
                echo "Will release using feenk-releaser ${FEENK_RELEASER_VERSION}"
            }
        }
        stage ('Parallel build') {
            parallel {
                stage ('MacOS x86_64') {
                    agent {
                        label "${MACOS_INTEL_TARGET}"
                    }

                    environment {
                        TARGET = "${MACOS_INTEL_TARGET}"
                        PATH = "$HOME/.cargo/bin:/usr/local/bin/:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo build --bin ${TOOL_NAME} --release"

                        sh "mv target/release/${TOOL_NAME} ${TOOL_NAME}-${TARGET}"

                        stash includes: "${TOOL_NAME}-${TARGET}", name: "${TARGET}"
                    }
                }
                stage ('MacOS M1') {
                    agent {
                        label "${MACOS_M1_TARGET}"
                    }

                    environment {
                        TARGET = "${MACOS_M1_TARGET}"
                        PATH = "$HOME/.cargo/bin:/opt/homebrew/bin:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo build --bin ${TOOL_NAME} --release"

                        sh "mv target/release/${TOOL_NAME} ${TOOL_NAME}-${TARGET}"

                        stash includes: "${TOOL_NAME}-${TARGET}", name: "${TARGET}"
                    }
                }

            }
        }

        stage ('Deployment') {
            agent {
                label "${MACOS_M1_TARGET}"
            }
            environment {
                TARGET = "${MACOS_M1_TARGET}"
                PATH = "$HOME/.cargo/bin:/usr/local/bin/:$PATH"
            }
            when {
                expression {
                    (currentBuild.result == null || currentBuild.result == 'SUCCESS') && env.BRANCH_NAME.toString().equals('main')
                }
            }
            steps {
                unstash "${MACOS_INTEL_TARGET}"
                unstash "${MACOS_M1_TARGET}"

                withCredentials([
                    file(credentialsId: 'feenk-apple-developer-certificate', variable: 'CERT'),
                    string(credentialsId: 'feenk-apple-signing-identity', variable: 'SIGNING_IDENTITY')
                ]) {
                    // sign both apps
                    sh "cargo run --release -- mac ${TOOL_NAME}-${MACOS_INTEL_TARGET}"
                    sh "cargo run --release -- mac ${TOOL_NAME}-${MACOS_M1_TARGET} "
                }

                sh "curl -o feenk-releaser -LsS  https://github.com/feenkcom/releaser-rs/releases/download/${FEENK_RELEASER_VERSION}/feenk-releaser-${TARGET}"
                sh "chmod +x feenk-releaser"
                sh """
                ./feenk-releaser \
                    --owner feenkcom \
                    --repo feenk-signer \
                    --token GITHUB_TOKEN \
                    release \
                    --bump ${params.BUMP} \
                    --auto-accept \
                    --assets \
                        ${TOOL_NAME}-${MACOS_INTEL_TARGET} \
                        ${TOOL_NAME}-${MACOS_M1_TARGET}  """

            }
        }
    }
}
