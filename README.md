# Aizel Network Overview

## Introduction

AizelNetwork is a network platform dedicated to providing a secure, transparent, and verifiable AI computing environment. Its core lies in ensuring the trustworthiness of AI inference tasks executed in a distributed environment, especially through the utilization of Trusted Execution Environments (TEE) to protect data privacy and the integrity of computational results. The architecture of AizelNetwork comprises several key components, with the Aizel Verifier Library serving as the cornerstone of its verification mechanism.

## Network Architecture

The architecture of AizelNetwork primarily consists of the following four key nodes:

1. **Gate Node**: Serving as the entry point for users to interact with AizelNetwork, the Gate Node is responsible for receiving user requests and assigning inference tasks to appropriate inference nodes based on the request type and load. It also facilitates timely communication with users to ensure the prompt return of inference results.

2. **Data Node**: The Data Node is responsible for storing various resources required for AI inference, including inference information, agent configurations, AI models, and more. These resources are crucial for the smooth operation of inference nodes.

3. **Inference Node**: The Inference Node performs AI inference tasks within a TEE. These nodes leverage hardware-level security guarantees, such as Intel SGX or AMD SEV, to ensure data privacy and the integrity of computational results during the inference process. Upon completion of an inference task, the Inference Node generates a remote attestation report for verification by the Verifier Node.

4. **Verifier Node**: The Verifier Node is responsible for validating the remote attestation reports generated by Inference Nodes. By comparing and verifying the information in these reports, the Verifier Node ensures the security and accuracy of the inference process.

## Aizel Verifier Library

### Overview

The Aizel Verifier Library serves as the core verification component of AizelNetwork, focusing on validating remote attestation reports generated by Inference Nodes. This library ensures the trustworthiness and privacy protection of AI computational results through a secure and transparent approach.

### Features

1. **Security**: Leverages TEE to provide hardware-level security guarantees, preventing data breaches and tampering of computational results.
2. **Transparency**: Enables users to gain a clear understanding of the key information of the inference process through remote attestation reports, building trust in AI computational results.
3. **Verifiability**: Ensures the security and accuracy of the inference process through the comparison and verification of reports by the Verifier Node.
4. **Ease of Deployment**: Provides concise deployment guidelines and tools, supporting multiple deployment methods to meet the needs of different users.

### Deployment and Testing

#### Deployment

Deploying the Aizel Verifier Library typically involves the following steps:

1. **Environment Preparation**:
   - Ensure that the system meets the minimum hardware requirements, such as a 4-core CPU and 8GB of RAM.
   - Install the Rust programming environment, including the Rust compiler (`rustc`) and the Cargo package manager.
   - Install any necessary library files or dependencies required by the Aizel Verifier Library.

2. **Code Cloning**:
   - Clone the code of the Aizel Verifier Library from Aizel's official repository using a version control system like Git.

3. **Project Building**:
   - Navigate to the verifier directory in the cloned repository.
   - Run the Cargo build command to compile the project.

   ```bash
   cd verifier && cargo build
   ```

4. **Node Configuration**:
   - Configure relevant parameters for each node (Gate Node, Data Node, Inference Node, Verifier Node) based on your requirements. This includes settings like network addresses, ports, encryption keys, and other security measures.

5. **Node Startup**:
   - Start the Gate Node, Data Node, Inference Node, and Verifier Node, ensuring that network connections are stable and configurations are correct. Monitor the logs for any errors or warnings.
  
  
#### Testing

Run test cases to verify the functionality and performance of the Aizel Verifier Library.

```bash
cargo test
```
Note: As the library is currently under development, additional testing and refinement may be required to ensure its stability and reliability. It's recommended to follow the development roadmap and documentation provided by Aizel for the latest information on deployment and testing requirements.


AizelNetwork, along with its core component, the Aizel Verifier Library, provides users with a secure, transparent, and verifiable AI computing environment, laying a solid foundation for the widespread application of AI technologies.
