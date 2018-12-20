{ stdenv, rustPlatform, darwin, fetchFromGitLab }:

with rustPlatform;

buildRustPackage rec {
	name = "website-${version}";
	version = "0.0.1";

	buildInputs = [ ]
	++ stdenv.lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];

	src = ./.;
	# Change this when it error's
	cargoSha256 = "1q85k1h1p9gi02lgqd6n6dfvl5nn219bsgd0gz11z8z3d0mxid35";

	meta = with stdenv.lib; {
		description = "Fuzen's websites";
		longDescription = "Fuzen's websites in one app";
		homepage = "https://gitlab.com/Fuzen-py/website";
		license = licenses.mit;
		maintainers = [ maintainers.Fuzen ];
		platforms = platforms.all;
	};

	doCheck = true;
}
