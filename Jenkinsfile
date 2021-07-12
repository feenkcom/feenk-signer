import hudson.tasks.test.AbstractTestResultAction
import hudson.model.Actionable
import hudson.tasks.junit.CaseResult

pipeline {
    agent none
    options {
        buildDiscarder(logRotator(numToKeepStr: '50'))
        disableConcurrentBuilds()
    }
    environment {
        GITHUB_TOKEN = credentials('githubrelease')
        AWSIP = 'ec2-18-197-145-81.eu-central-1.compute.amazonaws.com'

        TOOL_NAME = 'feenk-signer'
        MACOS_INTEL_TARGET = 'x86_64-apple-darwin'
        MACOS_M1_TARGET = 'aarch64-apple-darwin'
        WINDOWS_AMD64_TARGET = 'x86_64-pc-windows-msvc'
        LINUX_AMD64_TARGET = 'x86_64-unknown-linux-gnu'
    }

    stages {
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
                label "${MACOS_INTEL_TARGET}"
            }
            environment {
                TARGET = "${MACOS_INTEL_TARGET}"
                PATH = "$HOME/.cargo/bin:/usr/local/bin/:$PATH"
                CERT = credentials('devcertificate')
                APPLEPASSWORD = credentials('notarizepassword')

            }
            when {
                expression {
                    (currentBuild.result == null || currentBuild.result == 'SUCCESS') && env.BRANCH_NAME.toString().equals('main')
                }
            }
            steps {
//              unstash "${LINUX_AMD64_TARGET}"
                unstash "${MACOS_INTEL_TARGET}"
                unstash "${MACOS_M1_TARGET}"
//              unstash "${WINDOWS_AMD64_TARGET}"


                sh """
                cargo run --release -- --app ${TOOL_NAME}-${MACOS_INTEL_TARGET} \
                    --singing-identity "Developer ID Application: feenk gmbh (77664ZXL29)" \
                    --entitlements resources/Product.entitlements"""

                sh """
                cargo run --release -- --app ${TOOL_NAME}-${MACOS_M1_TARGET} \
                    --singing-identity "Developer ID Application: feenk gmbh (77664ZXL29)" \
                    --entitlements resources/Product.entitlements"""

                sh "wget -O feenk-releaser https://github.com/feenkcom/releaser-rs/releases/latest/download/feenk-releaser-${TARGET}"
                sh "chmod +x feenk-releaser"
                sh """
                ./feenk-releaser \
                    --owner feenkcom \
                    --repo feenk-signer \
                    --token GITHUB_TOKEN \
                    --bump-minor \
                    --auto-accept \
                    --assets \
                        ${TOOL_NAME}-${MACOS_INTEL_TARGET} \
                        ${TOOL_NAME}-${MACOS_M1_TARGET}  """

            }
        }
    }
}
