use crate::ast;

peg::parser! {
    pub grammar prog_parser() for str {
        rule _ = [' ' | '\t']*

        rule newline() = [' ' | '\t' | '\n']*

        rule number() -> i128
            = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        rule character() -> char
            = c:[ 'a'..='z' | 'A'..='Z' | '-']
                { c }

        rule name() -> ast::Name
            = n:character()+
                { n.into_iter().collect() }

        // sugar for "cat^2" -> "cat cat"
        rule name_sugar() -> Vec<ast::Name>
            = name:name() "^" num:number()
                { [name].into_iter().cycle().take(num as usize).collect() }
            / name:name()
                { vec![name] }

        rule rule_() -> ast::Rule
            = "::" _ left:(name_sugar()**_) _ ">" _ right:(name_sugar()**_)
                { ast::Rule::new(
                    left.into_iter().flatten().collect(),
                    right.into_iter().flatten().collect()
                ) }

        rule acc() -> Vec<ast::Name>
            = s:(name_sugar()++_) { s.into_iter().flatten().collect() }

        pub rule prog() -> ast::Prog
            = r:(rule_()**newline()) newline() s:acc()
                { ast::Prog::new(s, r) }
    }
}
