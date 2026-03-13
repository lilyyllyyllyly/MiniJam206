{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	nativeBuildInputs = with pkgs; [
		rustc
		cargo

		pkgs.llvmPackages.bintools # for lld when specifying target (maybe just wasm target?)
	];

	buildInputs = with pkgs; [
		xorg.libX11
		xorg.libXi

		libxkbcommon
		libGL
	];

	shellHook = with pkgs; ''
		export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${xorg.libX11}/lib:${libxkbcommon}/lib:${xorg.libXi}/lib:${libGL}/lib
	'';
}

