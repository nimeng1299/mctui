pub trait AccountBase {
    const KIND: &'static str;
    fn get_username(&self) -> &str;
    fn get_uuid(&self) -> &str;
    fn get_access_token(&self) -> &str;
    /// legacy  离线账户 / 旧账号
    ///
    /// mojang	Mojang 账号时代
    ///
    /// msa	    Microsoft 账号
    fn get_user_type(&self) -> &str;
}
