#!/usr/bin/env perl

use strict;
use utf8;
use Term::Drawille;

binmode STDOUT, ':encoding(utf8)';

my $canvas = Term::Drawille->new(
	width => 534,
	height => 534,
);

for(my $i = 0; $i < 534; $i++) {
	$canvas->set($i,$i,1);
}

print $canvas->as_string();
