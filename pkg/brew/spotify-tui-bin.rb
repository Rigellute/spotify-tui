class SpotifyTUIBin < Formula
  version '0.3.0'
  desc "A terminal user interface for Spotify"
  homepage "https://github.com/Rigellute/spotify-tui"

  if OS.mac?
    url "https://github.com/Rigellute/spotify-tui/releases/download/#{version}/spotify-tui-#{version}.tar.gz"
      sha256 "5e3df01a08efa960bd67d744edd0af9579e2930ce7af70a64dec8058047245ea"
  end

  def install
    bin.install "spt"
  end

  test do
    system "#{bin}/spt", "--version"
  end
end
