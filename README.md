Example Website written in Typescript/Angular2 and accompanying Rust server as a backend. 
Communicates over Http between Website and Rust server.

##### Install Rust Ubuntu 14.04, 15.04, 15.10

```bash
# install rust stable
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh

# install stable and make it default
sudo multirust update stable
sudo multirust default stable
```
##### Install Rust OSX with Homebrew

```bash
# install multirust
brew update
brew install multirust

# install stable and make it default
multirust update stable && multirust default stable
```
#### Starting the Rust Server

```bash
# download and build the rust server
git clone https://github.com/ddabek/rustsite-example
cd rustsite-example
cd rustserver
cargo run
```
Then open another console to start the Web Server


#### Starting the Node Server After Already Cloned Repo

```bash
# build the node server
cd rustsite-example
npm install
npm run build
node index.js
```


#Now navigate in a browser to "http://localhost:8000" and experience the rust.
