use std::{env, fs::File, io::Write, path::Path};

fn main() {
    let data = generate_swizzle_macro();
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("swizzle_macro.rs");
    let mut file = File::create(out_path).expect("Could not create swizzle_macro.rs");
    file.write_all(data.as_bytes())
        .expect("Could not write to swizzle_macro.rs");
}

fn generate_swizzle_macro() -> String {
    let mut match_arms = String::new();
    for i in 2..5 {
        match_arms += &generate_swizzle_macro_arm(i);
    }

    format!(
        "macro_rules! impl_swizzle {{
        {match_arms}
    }}"
    )
}

fn generate_fn_params(pattern: &Vec<char>) -> String {
    let mut out = "".to_string();

    for i in 0..pattern.len() {
        let char = pattern[i];
        out += &format!("self.{},", char);
    }

    out
}

fn generate_fn(components: &str, pattern: Vec<char>) -> String {
    let fn_params = generate_fn_params(&pattern);
    let fn_name = map_source(&components, &pattern);
    let len = pattern.len();

    format!(
        "\
    pub fn {fn_name}(&self) -> super::d{len}::Vec{len} {{
        super::d{len}::Vec{len}({fn_params})
        }}
    \n"
    )
    .to_string()
}

fn generate_swizzle_macro_arm(components: u32) -> String {
    let mut vec = Vec::new();
    let s = "xyzw".to_string();
    let component_vec = &s[0..components as usize];

    for i in 2..components + 1 {
        vec.extend(generate_swizzle_n(component_vec, i as u32));
    }

    let fns = vec.join("\n");

    format!(
        "\
        ($vector_type:ident, {components}) => {{
            impl $vector_type {{
                {fns}
            }}
        }};
    \n"
    )
}

fn map_source(components: &str, pattern: &Vec<char>) -> String {
    let chars: Vec<char> = components.chars().collect();
    pattern
        .iter()
        .map(|c| c.to_digit(10).unwrap())
        .map(|i| chars[i as usize])
        .collect()
}

fn generate_swizzle_n(components: &str, out_len: u32) -> Vec<String> {
    let b = components.len();
    let n = b.pow(out_len);

    (0..n)
        .map(|i| to_base(i as u32, b as u32, out_len as usize))
        .map(|s| generate_fn(&components, s.chars().collect()))
        .collect()
}

fn to_base(mut number: u32, base: u32, pad: usize) -> String {
    let mut result = String::new();

    while number > 0 {
        let remainder = number % base;
        result.push_str(&remainder.to_string());
        number /= base;
    }

    format!(
        "{:0>pad$}",
        result.chars().rev().collect::<String>(),
        pad = pad
    )
    .into()
}
