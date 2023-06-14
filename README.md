<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<h3 align="center">Brick Hill CrossOver Launcher</h3>
<div>
  <p align="center">
    A (browser-less) Brick Hill launcher designed for CrossOver.
    
 </p>
<center>
<img src="https://i.imgur.com/T1tTyzt.png" width="450" title="hover text">
</center>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
    <li><a href="#building">Building</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

This is a launcher for Brick Hill designed for CrossOver or other Wine bottles.

Using this, the game is fully playable on Mac and Linux under Wine without needing a web browser.

The launcher is written in Rust and gtk-rs. This is actually the first project I have written in Rust, so the code might not be the neatest.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

### Installation

Before you continue, make sure CrossOver is installed on your Mac or Linux device.

1. Go to the [Brick Hill](https://www.brick-hill.com/) website and either log into or register your account.
2. Go to the download page highlighted in the below photo.

<img src="https://i.imgur.com/LJcpxU3.png" width="350" title="hover text">

3. On the download page, download the legacy client.

<img src="https://i.imgur.com/MPcQdTt.png" width="350" title="hover text">

4. Open CrossOver, then go to Bottle -> New Bottle and name it something along the lines of "Brick Hill".
5. Install the legacy client onto the newly created bottle.
    * If you are on an Apple Silicon device and the installer quits immediately, you can open cmd on the bottle, put the path to the installer, and press enter. 
6. Go to the releases page and download the latest version's zip file.
7. Extract the launcher to the virtual C drive of your bottle.
8. Click Run Command and browse to "BrickHillCrossoverLauncher.exe", then click on "Save Command as a Launcher".
9. Double click the launcher from the bottle's app list.
10. When the launcher opens, click the "Launch Game" button to create the config file.
11. Using a cookie editor (such as [EditThisCookie](https://chrome.google.com/webstore/detail/editthiscookie/fngmhnnpilhplaeedifhccceomclgfbg)), get and copy your entire Brick Hill session cookie.
    * ***Do NOT share this cookie with anyone***, as it can be used to access your account.
12. Open the "config.json" file in the launcher's folder in your favorite text editor.
13. Replace "INSERT_YOURS_HERE" with your copied cookie.
14. Close the launcher.

### Joining a game

1. Find a game you want to play on the Brick Hill website and copy the URL.
2. Double click the launcher from the bottle's app list.
3. Paste (or manually enter) the link into the launcher and click "Launch Game".
4. If successful, Brick Hill will open up. Have fun!

<img src="https://i.imgur.com/dBlCUYT.png" width="550" title="hover text">


<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Building

You will need the following in order to build the launcher:

* A Windows VM or machine
* An install of MSYS2 with GTK4 and Rust.
  * [Follow this guide if you need to install it.](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html)

1. Clone this repository to an easy to remember folder.
2. Open a MSYS2 MingW-w64 shell and cd into the project folder.
3. Run `cargo b --release` to build the executable.
4. If you can't run the built program out of the gate, move the executable into a new folder and copy all of the .dll files found in msys2/mingw64/bin.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Cleaning up the code.
- [ ] Figure out a better way to get the session cookie.
- [ ] Add localhost support (for node-hill development)
- [ ] Add a favorite games list feature.

See the [open issues](https://github.com/github_username/repo_name/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Any contributions you make are **greatly appreciated**.

If you have a suggestion that would improve this project, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star if you find this useful! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- LICENSE -->
## License

Distributed under the BSD-3 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTACT -->
## Contact

My Brick Hill username is [ComradeNC](https://www.brick-hill.com/user/1111148/); if you see me on a game, feel free to say hi!

If you want to add me on Discord, my username is @unity3d. Friending you back is not guaranteed however.

Project Link: [https://github.com/ComradeNapCali/BrickHillCrossoverLauncher](https://github.com/ComradeNapCali/BrickHillCrossoverLauncher)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Mooshimity](), for making Brick Hill itself.
* [Rust](https://www.rust-lang.org/), for being a beloved programming language.
* [CodeWeavers](https://www.codeweavers.com/), for making CrossOver, and supporting Wine development.

<p align="right">(<a href="#readme-top">back to top</a>)</p>
