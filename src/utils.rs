pub fn refresh_token_generate(user_id: usize) -> String {
  format!("refresh_token_{}", user_id)
}
