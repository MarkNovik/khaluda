use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "שגיאת" => "Err",
        "בסדר" => "Ok",
        "מחרוזת" => "String",
        "מילון" => "HashMap",
        "בררת_מחדל" => "Default",
        "שגיאה" => "Error",
        "אולי" => "Option",
        "קיים" | "קיימת" => "Some",
        "כלום" => "None",
        "תוצאה" => "Result",
        "עצם" => "Self",
        "אוספים" => "collections",
        "הדפס_שורה" => "println",
        "הדפס" => "print",
        "צא" => "break",
        "אסינכ" => "async",
        "המתן_ל" => "await",
        "לולאה" => "loop",
        "זוז" => "move",
        "מכולה" => "crate",
        "קופסה" => "Box",
        "עזה" => "unreachable_code",
        "כמו" => "as",
        "תמידי" => "const",
        "תכונה" => "trait",
        "טיפוס" => "type",
        "שטחים" => "unsafe",
        "בתוך" => "in",
        "מ" => "from",
        "דינמי" => "dyn",
        "גלל" => "unwrap",
        "בבררת_מחדל" => "default",
        "כ_הפניה" => "as_ref",
        "קפ" => "io",
        "חיצוני" => "extern",
        "שקר" => "false",
        "פעולה" => "fn",
        "בעל" => "super",
        "להוסיף" => "insert",

        /*        "wieder" => "iter",
                "zu_wieder" => "into_iter",
                "zuordnen" => "map",
                "ausbreiten" => "flat_map",
                "falte" => "fold",
                "leeren" => "drain",
                "sammeln" => "collect",
                "finde" => "find",
                "nehme" => "take",
                "produkt" => "product",*/

        // ordering
        "תשווה_ל" => "cmp",
        "סידור" => "Ordering",
        "גדול_מ" => "Greater",
        "קטן_מ" => "Less",
        "שווה" => "Equal",
        "לקחת" | "למצאו" => "get",
        "לאפשר" => "allow",
        "יא_זין" | "באסה" => "panic",
        "מודול" => "mod",
        "משתנה" => "mut",
        "חדש" | "חדשה" => "new",
        "ש" => "where",
        "ל" | "בשביל" => "for",
        "hole_oder_füge_ein_mit" | "hole_oder_fuege_ein_mit" => "get_or_insert_with",
        "ראשית" => "main",
        "פומבי" => "pub",
        "החזר" => "return",
        "ממש" => "impl",
        "הפניה" => "ref",
        "התאם" => "match",
        "אם" => "if",
        "אחרת" => "else",
        "עצמי" => "self",
        "שיהיה" => "let",
        "סטטי" => "static",
        "מבנה" => "struct",
        "צפה" => "expect",
        "כול_דוד" => "while",
        "שמש" => "use",
        "בית" => "std",
        "תתאים" => "into",
        "אמת" => "true",
        "אפשרויות" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn khaluda(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}