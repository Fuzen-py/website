{ stdenv, rustPlatform, darwin, fetchFromGitLab }:

with rustPlatform;

buildRustPackage rec {
	name = "website-${version}";
	version = "0.0.1";

	buildInputs = [ ]
	++ stdenv.lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];

	src = ./.;
	# Change this when it error's
	cargoSha256 = "1r0y0drq229bf762nawdy41pdhlih3zdi3klrwmkb546ql4jd32a";

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
