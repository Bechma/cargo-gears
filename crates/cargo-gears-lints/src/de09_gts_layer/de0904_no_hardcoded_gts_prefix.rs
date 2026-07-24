extern crate rustc_ast;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_then;
use rustc_ast::{AttrKind, Attribute, Expr, ExprKind, MacCall, token::LitKind};
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_span::Span;

dylint_linting::declare_pre_expansion_lint! {
    /// DE0904: Do not hard-code the GTS ID prefix
    ///
    /// GTS IDs must be created with `gts_id!("<suffix>")`, so the
    /// `GTS_ID_PREFIX` build-time configuration remains effective. This is a
    /// pre-expansion lint: it inspects user-authored source before ToolKit's
    /// wrapper macros generate inventory and error-builder code.
    #[doc = include_str!("../../docs/de09_gts_layer/de0904_no_hardcoded_gts_prefix/README.md")]
    pub DE0904_NO_HARDCODED_GTS_PREFIX,
    Deny,
    "hard-coded GTS ID prefix; use gts_id!(\"<suffix>\") instead (DE0904)"
}

impl EarlyLintPass for De0904NoHardcodedGtsPrefix {
    fn check_attribute(&mut self, cx: &EarlyContext<'_>, attr: &Attribute) {
        let AttrKind::Normal(normal_attr) = &attr.kind else {
            return;
        };

        // Name-value attributes such as `#[doc = "..."]` contain an
        // expression and are covered by `check_expr`. List attributes may
        // contain values (for example `reason = "..."`) that do not reach
        // that callback, so inspect only those here.
        if normal_attr.item.meta_item_list().is_none() {
            return;
        }

        if source_contains_hardcoded_prefix(cx, attr.span) {
            emit_lint(cx, attr.span);
        }
    }

    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &Expr) {
        if let ExprKind::Lit(lit) = &expr.kind
            && matches!(lit.kind, LitKind::Str | LitKind::StrRaw(_))
            && lit.symbol.as_str().starts_with("gts.")
        {
            emit_lint(cx, expr.span);
        }
    }

    fn check_mac(&mut self, cx: &EarlyContext<'_>, mac_call: &MacCall) {
        if source_contains_hardcoded_prefix(cx, mac_call.span()) {
            emit_lint(cx, mac_call.span());
        }
    }
}

fn source_contains_hardcoded_prefix(cx: &EarlyContext<'_>, span: Span) -> bool {
    let Ok(source) = cx.sess().source_map().span_to_snippet(span) else {
        return false;
    };
    source.contains("\"gts.")
}

fn emit_lint(cx: &EarlyContext<'_>, span: Span) {
    span_lint_and_then(
        cx,
        DE0904_NO_HARDCODED_GTS_PREFIX,
        span,
        "hard-coded GTS ID prefix; use gts_id!(\"<suffix>\") instead (DE0904)",
        |diag| {
            diag.help("for example: gts_id!(\"cf.core.users.user.v1~\")");
        },
    );
}
