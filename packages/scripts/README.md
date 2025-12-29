# yargs-parser



The mighty option parser used by.




## Example

```sh
npm i yargs-parser --save
```


**Node.js:**

```js
import parser from 'yargs-parser'

const argv = parser('--foo=99 --bar=9987930', {
  string: ['bar']
})
console.log(argv)
```


## License

ISC
