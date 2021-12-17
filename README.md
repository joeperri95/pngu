# pngu

Stuff a secret message into your PNG files

## How to use it

The project can be used with cargo.

To hide a message in an image use the `encode` subcommand and pass in a `--message` and an `--input` file. An `--output` can be optionally specified

``` 
$ cargo run -- encode --input "funny-image.png" --message "Let's meet later and discuss our secret plans" --output "inconspicuous-funny-image.png"
```

Optionally the `--use-web` or `-w` flag can be used in place of the `--input` switch to download a random image from Imgur. CAUTION as this can contain NSFW images.

To retrieve a message from an encoded image, use the `decode` subcommand and pass in the `--image`

``` 
$ cargo run -- decode --input "inconspicuous-funny-image.png"  
Let's meet later and discuss our secret plans 
```

## Demo

https://user-images.githubusercontent.com/17062561/146496274-eedc414f-894b-4d86-b229-8a4de2f3aa31.mp4


## How it works

PNG files are composed of chunks, most of which contain information about the image or actual image data. There are chunks in the PNG standard that allow for text strings to be stored as metadata in the image. pngu will create one of those text chunks with your message hidden inside.

## Limitations

 Right now the message is in plaintext, anyone with a hex editor can see the tEXt chunk containing your message. The plan is to add a method to encrypt the message with someone else's public key before storing it in the chunk. In any case, I am not a security pro and this should not be used to try and evade any serious scrutiny.

 Some services, such as facebook messenger, strip out image metadata to save bandwidth and storage space. This tool is completely useless in cases like this.
