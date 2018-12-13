{ stdenv, rustPlatform, darwin, fetchFromGitLab }:

with rustPlatform;

buildRustPackage rec {
	name = "website-${version}";
	version = "0.0.1";

	buildInputs = [ ]
	++ stdenv.lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];

	src = ./.;
	# Change this when it error's
	cargoSha256 = "0pgwhqdcvf280qcas8l7ik3yr31dzmrjgsqhvwcrg0ijp2zl5439";

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
