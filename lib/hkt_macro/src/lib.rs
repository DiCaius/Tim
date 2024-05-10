extern crate proc_macro;

fn extract_generic_params(ast: &syn::DeriveInput, arity: usize) -> syn::Result<Vec<&syn::TypeParam>> {
    let mut type_params = Vec::new();

    for param in &ast.generics.params {
        if let syn::GenericParam::Type(ref type_param) = *param {
            type_params.push(type_param);
        }
    }

    if type_params.len() == arity {
        syn::Result::Ok(type_params)
    } else {
        syn::Result::Err(syn::Error::new_spanned(
            ast,
            format!(
                "Incorrect type parameter arity: Expected {arity:?}, Got {:?}",
                type_params.len()
            ),
        ))
    }
}

fn extract_generic_param_information<'param>(
    ast: &syn::DeriveInput,
    params: &[&'param syn::TypeParam],
    index: usize,
) -> syn::Result<(
    &'param syn::Ident,
    &'param syn::punctuated::Punctuated<syn::TypeParamBound, syn::token::Plus>,
)> {
    params.get(index).map_or_else(
        #[allow(clippy::panic)]
        || {
            syn::Result::Err(syn::Error::new_spanned(
                ast,
                format!("Generic type parameter not found: {index:?}"),
            ))
        },
        |value| syn::Result::Ok((&value.ident, &value.bounds)),
    )
}

fn impl_hkt1_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro::TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    extract_generic_params(ast, 1).and_then(|params| {
        let (a_name, a_bounds) = extract_generic_param_information(ast, &params, 0)?;

        syn::Result::Ok(proc_macro::TokenStream::from(quote::quote! {
            impl #impl_generics HKT1 for #name #ty_generics #where_clause {
                type A = #a_name;
                type With<T: #a_bounds> = #name<T>;
            }
        }))
    })
}

fn impl_hkt2_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro::TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    extract_generic_params(ast, 2).and_then(|params| {
        let (a_name, a_bounds) = extract_generic_param_information(ast, &params, 0)?;
        let (b_name, b_bounds) = extract_generic_param_information(ast, &params, 1)?;

        syn::Result::Ok(proc_macro::TokenStream::from(quote::quote! {
            impl #impl_generics HKT2 for #name #ty_generics #where_clause {
                type A = #a_name;
                type B = #b_name;
                type With<T1: #a_bounds, T2: #b_bounds> = #name<T1, T2>;
            }
        }))
    })
}

fn impl_hkt3_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro::TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    extract_generic_params(ast, 3).and_then(|params| {
        let (a_name, a_bounds) = extract_generic_param_information(ast, &params, 0)?;
        let (b_name, b_bounds) = extract_generic_param_information(ast, &params, 1)?;
        let (c_name, c_bounds) = extract_generic_param_information(ast, &params, 2)?;

        syn::Result::Ok(proc_macro::TokenStream::from(quote::quote! {
            impl #impl_generics HKT3 for #name #ty_generics #where_clause {
                type A = #a_name;
                type B = #b_name;
                type C = #c_name;
                type With<T1: #a_bounds, T2: #b_bounds, T3: #c_bounds> = #name<T1, T2, T3>;
            }
        }))
    })
}

fn impl_hkt4_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro::TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    extract_generic_params(ast, 4).and_then(|params| {
        let (a_name, a_bounds) = extract_generic_param_information(ast, &params, 0)?;
        let (b_name, b_bounds) = extract_generic_param_information(ast, &params, 1)?;
        let (c_name, c_bounds) = extract_generic_param_information(ast, &params, 2)?;
        let (d_name, d_bounds) = extract_generic_param_information(ast, &params, 3)?;

        syn::Result::Ok(proc_macro::TokenStream::from(quote::quote! {
            impl #impl_generics HKT4 for #name #ty_generics #where_clause {
                type A = #a_name;
                type B = #b_name;
                type C = #c_name;
                type D = #d_name;
                type With<T1: #a_bounds, T2: #b_bounds, T3: #c_bounds, T4: #d_bounds> = #name<T1, T2, T3, T4>;
            }
        }))
    })
}

fn impl_hkt5_macro(ast: &syn::DeriveInput) -> syn::Result<proc_macro::TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    extract_generic_params(ast, 5).and_then(|params| {
        let (a_name, a_bounds) = extract_generic_param_information(ast, &params, 0)?;
        let (b_name, b_bounds) = extract_generic_param_information(ast, &params, 1)?;
        let (c_name, c_bounds) = extract_generic_param_information(ast, &params, 2)?;
        let (d_name, d_bounds) = extract_generic_param_information(ast, &params, 3)?;
        let (e_name, e_bounds) = extract_generic_param_information(ast, &params, 4)?;

        syn::Result::Ok(proc_macro::TokenStream::from(quote::quote! {
            impl #impl_generics HKT5 for #name #ty_generics #where_clause {
                type A = #a_name;
                type B = #b_name;
                type C = #c_name;
                type D = #d_name;
                type E = #e_name;
                type With<T1: #a_bounds, T2: #b_bounds, T3: #c_bounds, T4: #d_bounds, T5: #e_bounds> = #name<T1, T2, T3, T4, T5>;
            }
        }))
    })
}

/// Macro for deriving implementations of [`hkt::HKT1`].
///
/// # Examples
/// ```
/// use hkt::HKT1
/// use hkt_macro:HKT1
///
/// #[derive(HKT1)]
/// enum HKT<A> {
///     T1(A),
/// }
/// ```
#[proc_macro_derive(HKT1)]
pub fn hkt1_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    impl_hkt1_macro(&ast).unwrap_or_else(|error| syn::Error::into_compile_error(error).into())
}

/// Macro for deriving implementations of [`hkt::HKT2`].
///
/// # Examples
/// ```
/// use hkt::HKT2
/// use hkt_macro:HKT2
///
/// #[derive(HKT2)]
/// enum HKT<A, B> {
///     T1(A),
///     T2(B),
/// }
/// ```
#[proc_macro_derive(HKT2)]
pub fn hkt2_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    impl_hkt2_macro(&ast).unwrap_or_else(|error| syn::Error::into_compile_error(error).into())
}

/// Macro for deriving implementations of [`hkt::HKT3`].
///
/// # Examples
/// ```
/// use hkt::HKT3
/// use hkt_macro:HKT3
///
/// #[derive(HKT3)]
/// enum HKT<A, B, C> {
///     T1(A),
///     T2(B),
///     T3(C),
/// }
/// ```
#[proc_macro_derive(HKT3)]
pub fn hkt3_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    impl_hkt3_macro(&ast).unwrap_or_else(|error| syn::Error::into_compile_error(error).into())
}

/// Macro for deriving implementations of [`hkt::HKT4`].
///
/// # Examples
/// ```
/// use hkt::HKT4
/// use hkt_macro:HKT4
///
/// #[derive(HKT4)]
/// enum HKT<A, B, C, D> {
///     T1(A),
///     T2(B),
///     T3(C),
///     T4(D),
/// }
/// ```
#[proc_macro_derive(HKT4)]
pub fn hkt4_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    impl_hkt4_macro(&ast).unwrap_or_else(|error| syn::Error::into_compile_error(error).into())
}

/// Macro for deriving implementations of [`hkt::HKT5`].
///
/// # Examples
/// ```
/// use hkt::HKT5
/// use hkt_macro:HKT5
///
/// #[derive(HKT5)]
/// enum HKT<A, B, C, D, E> {
///     T1(A),
///     T2(B),
///     T3(C),
///     T4(D),
///     T5(E),
/// }
/// ```
#[proc_macro_derive(HKT5)]
pub fn hkt5_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    impl_hkt5_macro(&ast).unwrap_or_else(|error| syn::Error::into_compile_error(error).into())
}
