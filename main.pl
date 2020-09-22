use Test2::V0;
use FindBin;
use lib "$FindBin::Bin/lib";

use Person;

my $plicease = Person->new("Graham Ollis", 42);

is $plicease->name, "Graham Ollis";
is $plicease->lucky_number, 42;

done_testing;
