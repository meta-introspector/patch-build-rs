use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

pub fn backpack_fill_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let items_data = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🎒 Solving context window knapsack problem");
            
            // Parse items: "item1:weight:value,item2:weight:value"
            let items: Vec<(&str, u32, u32)> = #items_data
                .split(',')
                .filter_map(|item| {
                    let parts: Vec<&str> = item.split(':').collect();
                    if parts.len() == 3 {
                        Some((parts[0], parts[1].parse().unwrap_or(1), parts[2].parse().unwrap_or(1)))
                    } else { None }
                })
                .collect();
            
            // Knapsack DP solution
            let capacity = 4096; // Context window size
            let n = items.len();
            let mut dp = vec![vec![0u32; capacity + 1]; n + 1];
            
            // Fill DP table
            for i in 1..=n {
                let (_, weight, value) = items[i-1];
                for w in 0..=capacity {
                    if weight <= w as u32 {
                        dp[i][w] = dp[i-1][w].max(dp[i-1][w - weight as usize] + value);
                    } else {
                        dp[i][w] = dp[i-1][w];
                    }
                }
            }
            
            let max_value = dp[n][capacity];
            
            let knapsack_result = format!(
                "KnapsackSolution {{ capacity: {}, items: {}, max_value: {}, efficiency: {:.2} }}",
                capacity, n, max_value, max_value as f64 / capacity as f64
            );
            
            println!("cargo:warning=📊 Optimal context: {} value in {} tokens", max_value, capacity);
            knapsack_result
        }
    }.into()
}

pub fn context_optimize_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let context_items = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🔍 Optimizing context window content");
            
            let optimization_code = format!(
                r#"
// Auto-generated Context Window Optimizer
pub struct ContextOptimizer {{
    pub window_size: usize,
    pub items: Vec<ContextItem>,
    pub selected: Vec<bool>,
}}

#[derive(Clone, Debug)]
pub struct ContextItem {{
    pub content: String,
    pub tokens: usize,
    pub importance: f64,
    pub recency: f64,
}}

impl ContextOptimizer {{
    pub fn new(window_size: usize) -> Self {{
        Self {{
            window_size,
            items: Vec::new(),
            selected: Vec::new(),
        }}
    }}
    
    pub fn add_item(&mut self, content: &str, tokens: usize, importance: f64) {{
        self.items.push(ContextItem {{
            content: content.to_string(),
            tokens,
            importance,
            recency: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap().as_secs() as f64,
        }});
    }}
    
    pub fn solve_knapsack(&mut self) -> Vec<String> {{
        let n = self.items.len();
        if n == 0 {{ return Vec::new(); }}
        
        // Dynamic programming knapsack solution
        let mut dp = vec![vec![0.0; self.window_size + 1]; n + 1];
        
        for i in 1..=n {{
            let item = &self.items[i-1];
            let weight = item.tokens;
            let value = item.importance * item.recency.log10();
            
            for w in 0..=self.window_size {{
                if weight <= w {{
                    dp[i][w] = dp[i-1][w].max(dp[i-1][w - weight] + value);
                }} else {{
                    dp[i][w] = dp[i-1][w];
                }}
            }}
        }}
        
        // Backtrack to find selected items
        let mut selected_items = Vec::new();
        let mut w = self.window_size;
        for i in (1..=n).rev() {{
            if dp[i][w] != dp[i-1][w] {{
                selected_items.push(self.items[i-1].content.clone());
                w -= self.items[i-1].tokens;
            }}
        }}
        
        selected_items
    }}
}}
                "#
            );
            
            optimization_code
        }
    }.into()
}

pub fn token_weight_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let content = input_str.value();
    
    quote! {
        {
            // Estimate token weight and information value
            let char_count = #content.len();
            let word_count = #content.split_whitespace().count();
            let token_estimate = (char_count as f64 / 4.0) as u32; // ~4 chars per token
            
            // Information value heuristics
            let has_code = #content.contains("fn ") || #content.contains("struct ");
            let has_numbers = #content.chars().any(|c| c.is_ascii_digit());
            let has_symbols = #content.chars().any(|c| "{}[]()".contains(c));
            
            let base_value = word_count as u32;
            let code_bonus = if has_code { base_value / 2 } else { 0 };
            let data_bonus = if has_numbers { base_value / 4 } else { 0 };
            let struct_bonus = if has_symbols { base_value / 8 } else { 0 };
            
            let total_value = base_value + code_bonus + data_bonus + struct_bonus;
            
            let weight_result = format!(
                "TokenWeight {{ tokens: {}, value: {}, density: {:.2} }}",
                token_estimate, total_value, total_value as f64 / token_estimate as f64
            );
            
            println!("cargo:warning=⚖️ Token analysis: {} tokens, {} value", token_estimate, total_value);
            weight_result
        }
    }.into()
}

pub fn context_compress_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let full_context = input_str.value();
    
    quote! {
        {
            println!("cargo:warning=🗜️ Compressing context for optimal packing");
            
            // Aggressive compression for context window
            let compressed = #full_context
                .lines()
                .filter(|line| !line.trim().is_empty())
                .filter(|line| !line.trim().starts_with("//"))
                .map(|line| line.trim())
                .filter(|line| line.len() > 3) // Remove tiny lines
                .collect::<Vec<_>>()
                .join(" ");
            
            // Further compression
            let ultra_compressed = compressed
                .replace("  ", " ")
                .replace(" { ", "{")
                .replace(" } ", "}")
                .replace(" ( ", "(")
                .replace(" ) ", ")");
            
            let compression_ratio = #full_context.len() as f64 / ultra_compressed.len() as f64;
            
            println!("cargo:warning=📉 Compression: {:.1}x ratio", compression_ratio);
            ultra_compressed
        }
    }.into()
}
