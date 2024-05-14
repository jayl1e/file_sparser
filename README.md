# Why

I have a vm image, it's raw format. So it take exact 64GiB, which ocupies a lot on my macbook. I use this software to decrease file capacity on disk.

# How

MacOS use apfs and it supports sparse file, this program just skip write the block if it is all zero.

# How to use it

```sh
file_sparser --src=test.src --dst=test.dst
diff test.src test.dst
```

if it is the same, just move and replace it
