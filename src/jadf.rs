pub mod jadf {
	pub struct Field {
		variable_name: String,
		value_string: String,
	}
	pub mod read {
		pub fn extract_var_name(field_string: &String, rule: &Regex) -> Option<String> {
		    //if it doesn't find a proper name, or the regex
		    //doesn't match, the function returns None
		    let mut var_name: String = String::new();
		    //it's dirty and i hate it, but idk about any better option
		    let mut index: usize = 0;
		    let mut has_met_chars: bool = false;
		    if rule.is_match(field_string) {
		        //the string contains the rule (regex)
		        //the next variable is meant to be used to later check if the name
		        //contains whitespaces, tabs, and so on. in which case, None is returned
		        for ch in field_string.chars() {
		            //it's not wise to use the .chars() method,
		            //but idk about other options so here we go:
		            if ch == ' ' || ch == '\t' {
		                if has_met_chars == true {
		                    return Some(var_name);
		                } else {
		                    continue;
		                }
		            } else if ch == '<' {
		                //i check if i will eventually find the regex again in the rest
		                //of the string. that is, i check if < is the start of an operator
		                //or if it's part of the name
		                if rule.is_match(&field_string[index..]) {
		                    has_met_chars = true;
		                    var_name.push(ch);
		                } else {
		                    return Some(var_name);
		                }
		            } else {
		                has_met_chars = true;
		                var_name.push(ch);
		            }
		            index += 1;
		        }
		        return Some(var_name)
		    } else {
		        return None
		    }
		}
		pub fn extract_var_value(field_string: &String, rule: &Regex) -> Option<String> {
		    //this is used to check if in the string aaa <- "bbb" i'm in the double
		    //quotes or not.
		    let mut is_in_dquotes: bool = false;
		    let mut var_value: String = String::new();
		    if rule.is_match(&field_string) {
		        for ch in field_string.chars() {
		            //check for the opening ":
		            if ch == '"' {
		                is_in_dquotes = !is_in_dquotes; //flip the variable
		                continue;
		            }
		            if is_in_dquotes == true {
		                //accumulate until you find another '"':
		                var_value.push(ch);
		            }
		        }
		        if is_in_dquotes == true {
		            //it was not able to find a closing '"':
		            return None;
		        }
		    } else {
		        return None
		    }
		    Some(var_value)
		}
		fn extract_lines(contents: &String) -> Vec<String> {
		    //this function returns a vector of strings, each containing a line of the
		    //original contents of the file
		    let mut vector: Vec<String> = Vec::new();
		    let mut temp = String::new();
		    for ch in contents.chars() {
		        if ch == '\n' {
		            //push temp into the vector, reset it, and continue reading:
		            vector.push(temp);
		            temp = "".to_string();
		        } else {
		            temp.push(ch);
		        }
		    }
		    vector
		}
	}
}
