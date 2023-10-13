# PrivatURL

A small program that puts an encrypted blob of data into an HTML page.
Upon presenting the correct key, the encrypted blob is interpreted as HTML and shown.

You can put private content on a public URL and share it via any other secure 
transport, such as Whatsapp or Signal.

Recipients can either put the key in a form or simply access `site.com/secret.html#{YOUR_KEY}` to view the content.

```
./privaturl 
Generate HTML that reveals data only with the correct passphrase

Usage: privaturl [OPTIONS] --data <FILE>

Options:
  -d, --data <FILE>
  -o, --output <OUTPUT>              [default: private.html]
  -a, --alt-template <ALT_TEMPLATE>  [optional] Use an alternative template. See the README for details.
  -h, --help                         Print help
  -V, --version                      Print version
```

You can optionally provide an alternative template with `-a`. See the **Template** section.

# Example

You can find an example [here](https://agaricus.xyz/encrypted.html).

The key is `YCe2PzENsCx5BRFGdfMF32Q6v5JEtO3SnUeXWRXo64Q`.

Note you can also access: `https://agaricus.xyz/encrypted.html#YCe2PzENsCx5BRFGdfMF32Q6v5JEtO3SnUeXWRXo64Q` directly.

The exact process is simple:

```
./privateurl -d example.html
Your key is: YCe2PzENsCx5BRFGdfMF32Q6v5JEtO3SnUeXWRXo64Q
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

## Is sharing the key via URL safe?

Passing sensitive information via the URL is generally frowned upon -- with good reason. 
In this case it is mostly OK though, although the key does get stored into the browser's history.

- We have no concerns over server logs as this is intended for static webpages
- The `Referer` must specifically **not** include the `fragment` part of the URL, as [per the spec](https://www.rfc-editor.org/rfc/rfc7231#section-5.5.2).

If you are worried about the key persisting in the recipients browser's history, don't share it directly 
and let the recipient know. 

# Template

The default template is a very simple form with some code to handle the fragment-key feature 
and some silly anime gifs. 

You can use an alternative template without building from source by specifying an 
`--alt-template`. You can use `template/index.html` as inspiration. 

There are two requirements:

Firstly, the template must have `---PLACEHOLDER-DATA---` and a `PLACEHOLDER-IV---` 
so that privaturl knows where to inject the encrypted contents. 

Secondly, you should take into account that the encrypted data, initialization vector 
and provided key will be encoded using a variant of base64 which is URL-safe (no `/`, '+')
and without padding. 


Again, looking at `template/index.html` should be helpful. Look at the `urlSafeToBase64` function 
to see an example of how to handle the encoding specifically.

