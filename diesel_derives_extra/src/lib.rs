#![recursion_limit = "256"]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Meta, MetaList, NestedMeta};

#[proc_macro_derive(Model)]
pub fn model_macro_derive(input: TokenStream) -> TokenStream {
    // Represent our code as an AST
    let ast = syn::parse(input).unwrap();
    impl_model_macro(&ast)
}

fn impl_model_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
    impl<'a> ::diesel_derives_traits::Model<'a> for #name
    {
        fn save(self, conn: &mut ::diesel::pg::PgConnection) -> ::diesel::result::QueryResult<Self> {
            ::diesel::RunQueryDsl::get_result(
                ::diesel::update(
                    &self
                )
                .set(&self),
                conn
            )
            .map_err(|e| e.into())
        }

        fn find_all(conn: &mut ::diesel::pg::PgConnection) -> ::diesel::result::QueryResult<Vec<Self>> {
            ::diesel::RunQueryDsl::load(
                <Self as ::diesel::associations::HasTable>::table(),
                conn
            )
            .map_err(|e| e.into())
        }

        fn find_one(conn: &mut ::diesel::pg::PgConnection, id: <&'a Self as ::diesel::Identifiable>::Id) -> ::diesel::result::QueryResult<Option<Self>> {
            use diesel::{OptionalExtension, QueryDsl};

            ::diesel::RunQueryDsl::get_result(
                <Self as ::diesel::associations::HasTable>::table().find(id),
                conn
            )
            .optional()
            .map_err(|e| e.into())
        }

        fn exists(conn: &mut ::diesel::pg::PgConnection, id: <&'a Self as ::diesel::associations::Identifiable>::Id) -> ::diesel::result::QueryResult<bool> {
            use diesel::QueryDsl;

            ::diesel::RunQueryDsl::get_result(
                ::diesel::select(::diesel::dsl::exists(<Self as ::diesel::associations::HasTable>::table().find(id))),
                conn
            )
            .map_err(|e| e.into())
        }

        fn count_all(conn: &mut ::diesel::pg::PgConnection) -> ::diesel::result::QueryResult<i64> {
            use diesel::QueryDsl;

            ::diesel::RunQueryDsl::get_result(
                <Self as ::diesel::associations::HasTable>::table().count(),
                conn
            )
            .map_err(|e| e.into())
            }

        fn destroy(self, conn: &mut ::diesel::pg::PgConnection) -> ::diesel::result::QueryResult<()> {
            ::diesel::RunQueryDsl::execute(
                ::diesel::delete(&self),
                conn
            )?;
            Ok(())
            }
        }};

    gen.into()
}

#[proc_macro_derive(NewModel, attributes(model))]
pub fn new_model_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_new_model_macro(&ast)
}

fn impl_new_model_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut tokens = TokenStream2::new();
    name.to_tokens(&mut tokens);
    ast.generics.to_tokens(&mut tokens);
    let target = ast
        .attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == "model")
        .expect("\"model\" attribute must be specified for #[derive(NewModel)]");
    let target_name = match target
        .parse_meta()
        .expect("Must be in the form of `#[model(MyModel)]`")
    {
        Meta::List(MetaList { ref nested, .. }) if !nested.is_empty() => match nested[0] {
            NestedMeta::Meta(Meta::Path(ref ident)) => ident.clone(),
            _ => panic!("Must be in the form of `#[model(MyModel)]`"),
        },
        _ => panic!("Must be in the form of `#[model(MyModel)]`"),
    };

    let gen = {
        quote!(
            impl<'a> ::diesel_derives_traits::NewModel<'a, #target_name> for #tokens
            {
                fn save(self, conn: &mut ::diesel::pg::PgConnection) -> ::diesel::result::QueryResult<#target_name> {
                    ::diesel::RunQueryDsl::get_result(
                        ::diesel::insert_into(<#target_name as ::diesel::associations::HasTable>::table())
                            .values(&self),
                        conn
                    )
                    .map_err(|e| e.into())
                }
            }
        )
    };
    gen.into()
}
