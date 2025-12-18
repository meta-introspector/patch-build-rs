extern crate proc_macro;

// Import the declarative macro from our core library
use emoji_macros_core::define_emoji_macro;
use quote::quote; // Required for the macro expansion
use syn::{parse_macro_input, ItemStruct}; // Required for the macro expansion

// This lib.rs will now contain calls to define_emoji_macro! for each
// individual emoji macro, making their definitions concise.

define_emoji_macro! {
    fork_knife, // macro_name_ident
    fork_count: u64, // field_name: field_type
    { // methods
        pub fn create_fork(&mut self) {
            self.fork_count += 1;
            self.energy += 12; // Creating a fork expands possibilities, increasing energy
            println!("🍴 Fork created. Total forks: {}. Energy: {}", self.fork_count, self.energy);
        }

        pub fn merge_fork(&mut self) {
            if self.fork_count > 0 {
                self.fork_count -= 1;
                self.energy += 8; // Merging forks brings stability and energy
                println!("🤝 Fork merged. Total forks: {}. Energy: {}", self.fork_count, self.energy);
            } else {
                println!("⚠️ No forks to merge!");
            }
        }
    }
}

define_emoji_macro! {
    magnifying_glass,
    truth_revealed: bool,
    {
        pub fn seek_truth(&mut self) {
            self.truth_revealed = true;
            self.energy += 25; // Seeking truth brings clarity and energy
            println!("🔎 Truth revealed! Energy: {}", self.energy);
        }

        pub fn obscure_truth(&mut self) {
            self.truth_revealed = false;
            self.energy = self.energy.saturating_sub(15); // Obscuring truth can be costly
            println!("🙈 Truth obscured. Energy: {}", self.energy);
        }
    }
}

define_emoji_macro! {
    money,
    assets: u64,
    {
        pub fn acquire_assets(&mut self, amount: u64) {
            self.assets += amount;
            self.energy += amount / 10; // Assets contribute to energy
            println!("💰 Acquired {} assets. Total assets: {}. Energy: {}", amount, self.assets, self.energy);
        }

        pub fn deploy_assets(&mut self, amount: u64) {
            if self.assets >= amount {
                self.assets -= amount;
                self.energy = self.energy.saturating_sub(amount / 20); // Deploying assets costs some energy
                println!("💸 Deployed {} assets. Remaining assets: {}. Energy: {}", amount, self.assets, self.energy);
            } else {
                println!("⚠️ Not enough assets to deploy {}!", amount);
            }
        }
    }
}

define_emoji_macro! {
    herb,
    active_branches: Vec<String>,
    {
        pub fn create_branch(&mut self, branch_name: &str) {
            self.active_branches.push(branch_name.to_string());
            self.energy += 10; // New branches increase energy
            println!("🌿 Branch '{}' created. Active branches: {:?}. Energy: {}", branch_name, self.active_branches, self.energy);
        }

        pub fn switch_branch(&mut self, branch_name: &str) {
            if self.active_branches.contains(&branch_name.to_string()) {
                self.energy += 2; // Switching branches is a low-cost operation
                println!("🌱 Switched to branch '{}'. Energy: {}", branch_name, self.energy);
            } else {
                println!("⚠️ Branch '{}' not found. Cannot switch.", branch_name);
            }
        }
    }
}

define_emoji_macro! {
    bullseye,
    current_goal: Option<String>,
    {
        pub fn set_goal(&mut self, goal: &str) {
            self.current_goal = Some(goal.to_string());
            println!("🎯 Goal set: {}", goal);
            self.energy -= 10; // Setting a goal costs a little energy
        }

        pub fn is_goal_reasonable(&self) -> bool {
            if let Some(goal) = &self.current_goal {
                // This is a simple heuristic for "reasonableness"
                let reasonable = self.energy > 100 && self.complexity > 2 && self.assets > 50;
                if reasonable {
                    println!("✅ Goal '{}' seems reasonable given current state. Energy: {}, Complexity: {}, Assets: {}", goal, self.energy, self.complexity, self.assets);
                } else {
                    println!("❌ Goal '{}' might not be reasonable yet. Energy: {}, Complexity: {}, Assets: {}", goal, self.energy, self.complexity, self.assets);
                }
                reasonable
            } else {
                println!("🤷 No goal set to evaluate reasonableness.");
                false
            }
        }
    }
}

define_emoji_macro! {
    truck,
    vendored_deps: Vec<String>,
    {
        pub fn vendor_dependency(&mut self, dep_name: &str) {
            self.vendored_deps.push(dep_name.to_string());
            self.energy += 7; // Vendoring dependencies stabilizes the system, increasing energy
            println!("🚚 Vendored dependency: {}. Total vendored: {}. Energy: {}", dep_name, self.vendored_deps.len(), self.energy);
        }

        pub fn remove_vendored_dependency(&mut self, dep_name: &str) {
            let initial_len = self.vendored_deps.len();
            self.vendored_deps.retain(|d| d != dep_name);
            if self.vendored_deps.len() < initial_len {
                self.energy = self.energy.saturating_sub(5); // Removing vendored dependency can introduce instability
                println!("🗑️ Removed vendored dependency: {}. Remaining: {}. Energy: {}", dep_name, self.vendored_deps.len(), self.energy);
            } else {
                println!("⚠️ Vendored dependency {} not found to remove!", dep_name);
            }
        }
    }
}