use pyo3::prelude::*;

pub fn get_llm_output(input: &str) -> PyResult<(String)> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let script = r#"
def llm(input_text):
    import torch
    from transformers import AutoTokenizer, AutoModelForCausalLM
    model_name = "gpt2"

    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModelForCausalLM.from_pretrained(model_name)

    inputs = tokenizer(input_text, return_tensors="pt")

    output_ids = model.generate(inputs['input_ids'], max_length=50)

    generated_text = tokenizer.decode(output_ids[0], skip_special_tokens=True)

    return generated_text
"#;
        let module = PyModule::from_code_bound(
            py,
            script,
            "model.py",
            "llm",
        )?;

        let result: String = module
            .getattr("llm")?
            .call1((input,))?
            .extract()?;
        println!("Result: {result}");

        Ok((result))
    })
}
