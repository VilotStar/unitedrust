use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "として" => "as",
        "壊す" => "break",
        "絶え間ない" => "const",
        "続く" => "continue",
        "木箱" => "crate",
        "さもないと" => "else",
        "列挙" => "enum",
        "外部の" => "extern",
        "間違い" => "false",
        "関数" => "fn",
        "のために" => "for",
        "もし" => "if",
        "埋め込む" => "impl",
        "で" => "in",
        "させて" => "let",
        "ループ" => "loop",
        "マッチ" => "match",
        "変調" => "mod",
        "動く" => "move",
        "可変" => "mut",
        "公共" => "pub",
        "参照" => "ref",
        "戻る" => "return",
        "自己価値" => "self",
        "静的" => "static",
        "構造" => "struct",
        "素晴らしい" => "super",
        "特性" => "trait",
        "真実" => "true",
        "タイプ" => "type",
        "危険な" => "unsafe",
        "使用" => "use",
        "どこ" => "where",
        "その間" => "while",
        "非同期" => "async",
        "待つ" => "await",
        "動的" => "dyn",
        "抽象的な" => "abstract",
        "なる" => "become",
        "ボックスの値" => "box",
        "する" => "do",
        "最後の" => "final",
        "大きい" => "macro",
        "オーバーライド" => "override",
        "プライベート" => "priv",
        "の種類" => "typeof",
        "ないサイズのある" => "unsized",
        "バーチャル" => "virtual",
        "収率" => "yield",
        "試す" => "try",
        "マクロルール" => "macro_rules",
        "連合" => "union",
        "ブール値" => "bool",
        "u番号" => "usize",
        "i番号" => "isize",
        "文字" => "char",
        "文字列値" => "str",
        "箱" => "Box",
        "参カ" => "Rc",
        "原参カ" => "Arc",
        "ピン" => "Pin",
        "危険なセル" => "UnsafeCell",
        "ファントムデータ" => "PhantomData",
        "逆参照" => "Deref",
        "逆参照可変" => "DerefMut",
        "落とす" => "Drop",
        "コピー" => "Copy",
        "クローン" => "Clone",
        "送信" => "Send",
        "同期" => "Sync",
        "終了" => "Termination",
        "ないピン" => "Unpin",
        "サイズあり" => "Sized",
        "弦" => "String",
        "パニック" => "panic",
        "文字列を含める" => "include_str",
        "スプリット" => "split",
        "で分割" => "split_at",
        "集める" => "collect",
        "派生する" => "derive",
        "ベクター" => "Vec",
        "押す" => "push",
        "包みを解く" => "unwrap",
        "デフォルト値" => "default",
        "デフォルト" => "Default",
        "自己" => "Self",
        "の中へ" => "into",
        "印刷する" => "print",
        "印刷するln" => "println",
        "和" => "sum",
        "解析する" => "parse",
        "なし" => "None",
        "いくつかの" => "Some",
        "オプション" => "Option",
        "エラー" => "Err",
        "エラーVal" => "Error",
        "わかりました" => "Ok",
        "ハッシュ地図" => "HashMap",
        "結果" => "Result",
        "到達不能なコード" => "unreachable_code",
        "から" => "from",
        "参照として" => "as_ref",
        "得る" => "get",
        "許可する" => "allow",
        "取得または挿入" => "get_or_insert_with",
        "主要" => "main",
        "期待する" => "expect",
        "伸ばす" => "extend",
        "プロシージャマクロ" => "proc_macro",
        "列挙する" => "enumerate",
        "長さ" => "len",
        "キャラクター" => "chars",
        "フィルター" => "filter",
        "含まれています" => "contains",
        "n番目" => "nth",
        "小文字です" => "is_ascii_lowercase",
        "大文字です" => "is_ascii_uppercase",
        "文字列に" => "to_string",
        "地図" => "map",
        "可変スライスとして" => "as_mut_slice",
        "スライスとして" => "as_slice",
        "選別" => "sort",
        "逆行する" => "reverse",
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
pub fn sabi(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
