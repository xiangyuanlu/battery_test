importScripts('./xlsx.mini.js')
self.onmessage = function(event) {
  if (event.data instanceof File) {
    var reader = new FileReader()
    reader.readAsBinaryString(event.data)
    reader.onload = function(e) {
      var xlsxData = XLSX.read(e.target.result, { type: 'binary' })
      Object.keys(xlsxData.Sheets).forEach(function(key) {
        var csv = XLSX.utils.make_csv(xlsxData.Sheets[key])
        self.postMessage({csv: csv, name: key})
      })
      self.postMessage('over')
    }
  }
  if (event.data === 'close') {
    self.close()
  }
}
self.onerror = function(e) {
  console.error('worker appear exception', e)
}