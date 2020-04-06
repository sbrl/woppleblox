/**
 * Removes boilerplate when translating a string.
 */
#[macro_export]
macro_rules! translate {
    ( $gs: ident, $req: ident, $code: expr ) => ( &$gs.tr.translate_simple(
        &$req.get_req_lang(),
        $code
    ).unwrap() );
    ( $gs: ident, $req: ident, $code: expr, $args: ident ) => ( &$gs.tr.translate(
        &$req.get_req_lang(),
        $code,
        $args
    ).unwrap() );
}
