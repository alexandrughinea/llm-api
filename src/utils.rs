use llm::ModelArchitecture;

pub fn match_model_architecture(input: &str) -> Option<ModelArchitecture> {
    match input {
        "bloom" => Some(ModelArchitecture::Bloom),
        "gpt2" => Some(ModelArchitecture::Gpt2),
        "gptj" => Some(ModelArchitecture::GptJ),
        "gptneox" => Some(ModelArchitecture::GptNeoX),
        "llama" => Some(ModelArchitecture::Llama),
        "mpt" => Some(ModelArchitecture::Mpt),
        //"falcon" => Some(ModelArchitecture::Falcon),
        _ => None,
    }
}
