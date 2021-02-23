use titlecase::titlecase;

pub fn to_account_name<T: AsRef<str>>(formatted: T) -> String {
   formatted.as_ref().to_lowercase().replace(" ", "_")
}

pub fn to_formatted_name<T: AsRef<str>>(name: T) -> String {
   titlecase(name.as_ref().replace("_", " ").as_str())
}