use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    BinOp, Expr, ExprBinary, Ident, Token,
};

#[proc_macro]
pub fn define_binding(input: TokenStream) -> TokenStream {
    let binding_input = syn::parse_macro_input!(input as KeyButtonBinding);

    let name = &binding_input.struct_name;
    let keys = &binding_input.keys;
    let buttons = &binding_input.buttons;

    quote! {
        pub struct #name;

        impl isle_engine::input::Mapping for #name {
            fn keys<'a>() -> &'a [isle_engine::input::Key] {
                &[#(isle_engine::input::#keys),*]
            }

            fn buttons<'a>() -> &'a [isle_engine::input::Button] {
                &[#(isle_engine::input::#buttons),*]
            }
        }
    }
    .into()
}

struct KeyButtonBinding {
    struct_name: Ident,
    keys: Vec<Expr>,
    buttons: Vec<Expr>,
}

impl Parse for KeyButtonBinding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut items = Vec::new();

        let expr = input.parse::<Expr>()?;

        collect_list(&expr, &mut items)?;

        let mut keys = Vec::new();
        let mut buttons = Vec::new();

        for item in items {
            parse_single_binding(&item, &mut keys, &mut buttons)?;
        }

        Ok(Self {
            struct_name,
            keys,
            buttons,
        })
    }
}

#[proc_macro]
pub fn define_axis_binding(input: TokenStream) -> TokenStream {
    let binding_input = syn::parse_macro_input!(input as AxisBinding);

    let name = &binding_input.struct_name;
    let axes = &binding_input.axes;
    let positive_fallback = &binding_input.positive_fallback;
    let negative_fallback = &binding_input.negative_fallback;

    quote! {
        pub struct #name;

        impl isle_engine::input::AxisMapping for #name {
            type PositiveMapping = #positive_fallback;
            type NegativeMapping = #negative_fallback;

            fn axes<'a>() -> &'a [isle_engine::input::Axis] {
                &[#(isle_engine::input::#axes),*]
            }
        }
    }
    .into()
}

struct AxisBinding {
    struct_name: Ident,
    positive_fallback: Ident,
    negative_fallback: Ident,
    axes: Vec<Expr>,
}

impl Parse for AxisBinding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let struct_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let axes_expr: Expr = input.parse()?;
        let mut axes = Vec::new();
        collect_list(&axes_expr, &mut axes)?;
        input.parse::<Token![,]>()?;

        let positive_fallback: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let negative_fallback: Ident = input.parse()?;

        Ok(Self {
            struct_name,
            positive_fallback,
            negative_fallback,
            axes,
        })
    }
}

fn collect_list(expr: &Expr, mut list: &mut Vec<Expr>) -> syn::Result<()> {
    match expr {
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) if matches!(op, BinOp::BitOr(_)) => {
            collect_list(left, &mut list)?;
            collect_list(right, &mut list)?;
        }
        Expr::Path(_) => {
            list.push(expr.clone());
        }
        _ => {
            return Err(syn::Error::new_spanned(
                expr,
                "Expected path or binary expression",
            ))
        }
    }

    Ok(())
}

fn parse_single_binding(
    expr: &Expr,
    keys: &mut Vec<Expr>,
    buttons: &mut Vec<Expr>,
) -> syn::Result<()> {
    let path = if let Expr::Path(path) = expr {
        &path.path
    } else {
        return Err(syn::Error::new_spanned(expr, "Expected path"));
    };

    let ty = path
        .segments
        .first()
        .unwrap_or_else(|| panic!("Expected Key:: or Button::"));

    match ty.ident.to_string().as_str() {
        "Key" => keys.push(expr.clone()),
        "Button" => buttons.push(expr.clone()),
        _ => return Err(syn::Error::new_spanned(expr, "Expected Key:: or Button::")),
    }

    Ok(())
}
