package FastDecimate;

use strict;
use warnings;
use 5.014;
use FFI::Platypus 1.00;
use Path::Tiny qw( path );

our $VERSION = '1.00';

my $lib_file = path(__FILE__)->parent->child('target/release/libperson.so')->absolute;

my $ffi = FFI::Platypus->new( api => 1, lang => 'Rust' );
$ffi->lib($lib_file);

# use the FastDecimate_ prefix
$ffi->mangler(sub {
    my $symbol = shift;
    return "FastDecimate_$symbol";
});

$ffi->type( 'object(FastDecimate)' => 'decimate_t' );

$ffi->attach( new          => [ 'string' ] => 'decimate_t' );
$ffi->attach( DESTROY      => [ 'decimate_t' ] );
$ffi->attach( spot_min_max => [ 'decimate_t', 'string', 'u64', 'u64' ] => 'string' );

1;
