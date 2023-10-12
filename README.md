# PrivatURL

A small program that puts an encrypted blob of data into an HTML page.
Upon presenting the correct key, the encrypted blob is interpreted as HTML and shown.

You can put private content on a public URL and share it via any other secure 
transport, such as Whatsapp or Signal.

```
./privateurl 

Generate HTML that reveals data only with the correct passphrase

Usage: privaturl [OPTIONS] --data <FILE>

Options:
  -d, --data <FILE>
  -o, --output <OUTPUT>  [default: private.html]
  -h, --help             Print help
  -V, --version          Print version
```


# Example

You can find an example [here](https://agaricus.xyz/encrypted.html).

The key is `a9xh4KXk6nofBsEaVKvzMW4HlW68PpXuBF8bT6pDb2U=`.

The exact process is simple:

```
./privateurl -d example.html
Your key is: a9xh4KXk6nofBsEaVKvzMW4HlW68PpXuBF8bT6pDb2U=
Your output HTML is at: private.html
```

Then upload to wherever you want.

# Is this safe? 

Kind of. I designed it to pass stupid private jokes and memes 
to my girlfriend; so you know what you are buying. 

The encryption itself is safe. It uses [aes_gcm](https://docs.rs/aes-gcm/latest/aes_gcm/)
for the Rust program to encrypt the data; which has been audited. On the browser side, 
it uses the [Web Crypto API](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto) 
to dencrypt. It uses the OS Rng to get a key and a nonce. The nonce goes to the HTML, it is public. 
The key protects access to the webpage.

That being said: it takes absolutely no care or consideration about the private key 
security and how you distribute it. Once you generate it, it is printed 
in the console. Any malware can get hold of it. Once you pass it to someone else, 
it is of course up to them how they treat it and distribute it. 

All in all: if you are thinking about using this to protect serious, sensitive,  private data, don't. 
It is probably useless anyway, why would you want to embeded that data into an HTML? 
If you wanna create private static webpages to share around without someone looking into them 
without they key... then yes! 

