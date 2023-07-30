# What is it?
web-drain is a tool that assists with importing CSS and JS files into HTML webpages. Instead of importing the whole file, you can just import the parts the you need.

# How to use
web-drain has a pretty simple syntax.
Format it as follows
```
web-drain your-file what-you-want-to-import
```
Example:
```
web-drain main.css button
```

# Slight issue with JS files
Because of how web-drain works, any code located outside of a function (or any other block of code formatted with curly brackets '{}') will be shipped along with the next function.
