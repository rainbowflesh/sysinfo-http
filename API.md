# sysinfo-http API document

1. [sysinfo-http API document](#sysinfo-http-api-document)
   1. [Sysinfo API](#sysinfo-api)
      1. [CPU Stats](#cpu-stats)
         1. [Try it](#try-it)
         2. [Success](#success)
         3. [Error](#error)
      2. [Disks Stats](#disks-stats)
         1. [Try it](#try-it-1)
         2. [Success](#success-1)
         3. [Error](#error-1)
      3. [Memory Stats](#memory-stats)
         1. [Try it](#try-it-2)
         2. [Success](#success-2)
         3. [Error](#error-2)
      4. [Network](#network)
         1. [Try it](#try-it-3)
         2. [Success](#success-3)
         3. [Error](#error-3)
      5. [Temperatures](#temperatures)
         1. [Try it](#try-it-4)
         2. [Success](#success-4)
         3. [Error](#error-4)
      6. [Load Average](#load-average)
         1. [Try it](#try-it-5)
         2. [Success](#success-5)
         3. [Error](#error-5)
      7. [Boot Time](#boot-time)
         1. [Try it](#try-it-6)
         2. [Success](#success-6)
         3. [Error](#error-6)
      8. [System Infomation aka `uname -a`](#system-infomation-aka-uname--a)
         1. [Try it](#try-it-7)
         2. [Success](#success-7)
         3. [Error](#error-7)
      9. [Users](#users)
         1. [Try it](#try-it-8)
         2. [Success](#success-8)
         3. [Error](#error-8)
   2. [Common API](#common-api)
      1. [Root path](#root-path)
         1. [Response](#response)
      2. [404](#404)
         1. [Response](#response-1)
      3. [500](#500)
         1. [Response](#response-2)
      4. [health](#health)
         1. [Response](#response-3)
      5. [support](#support)
         1. [Response](#response-4)
      6. [teapot](#teapot)
         1. [Response](#response-5)

## Sysinfo API

### CPU Stats

> request{method=GET uri=/cpus version=HTTP/1.1}

#### Try it

```bash
curl -X GET '127.0.0.1:8000/cpus'
```

#### Success

- HTTPS status code: `200`
- Response example:

  ```json
  {
    "cpu_info": [
      { "cpu_num": "cpu0", "percent": 15.789473, "frequency": 4145 },
      { "cpu_num": "cpu1", "percent": 31.578945, "frequency": 3777 }
    ]
  }
  ```

#### Error

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

### Disks Stats

> request{method=GET uri=/disks version=HTTP/1.1}

#### Try it

```bash
curl -X GET '127.0.0.1:8000/disks'
```

#### Success

- HTTPS status code: `200`
- Response example:

  ```json
  [
    {
      "device_name": "/dev/nvme0n1p10",
      "file_system": "btrfs",
      "total_space": 353342849024,
      "available_space": 248559841280
    }
  ]
  ```

- unit: `byte`

#### Error

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

### Memory Stats

> request{method=GET uri=/memory version=HTTP/1.1}

#### Try it

```bash
curl -X GET '127.0.0.1:8000/memory'
```

#### Success

- HTTPS status code: `200`
- Response example:

  ```json
  [
    {
      "available_memory": 5858025472,
      "free_memory": 470913024,
      "free_swap": 14318530560,
      "total_memory": 16066314240,
      "total_swap": 17179865088,
      "used_memory": 10208288768,
      "used_swap": 2861334528
    }
  ]
  ```

- unit: `byte`

#### Error

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

### Network

> request{method=GET uri=/networks version=HTTP/1.1}

#### Try it

```bash
curl -X GET '127.0.0.1:8000/networks'
```

#### Success

- HTTPS status code: `200`
- Response example:

  ```json
  [
    { "interface_name": "wlp1s0", "data_received": 927, "data_transmitted": 281 },
    { "interface_name": "lo", "data_received": 0, "data_transmitted": 0 }
  ]
  ```

- unit: `byte`

#### Error

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

### Temperatures

> request{method=GET uri=/temperatures version=HTTP/1.1}

#### Try it

```bash
curl -X GET '127.0.0.1:8000/temperatures'
```

#### Success

- HTTPS status code: `200`
- Response example:

  ```json
  {
    "temperature_info": [
      {
        "name": "acpitz temp1",
        "temperature": 78.0
      },
      {
        "name": "acpitz temp2",
        "temperature": 30.0
      },
      {
        "name": "amdgpu edge",
        "temperature": 67.0
      },
      {
        "name": "k10temp Tctl",
        "temperature": 78.625
      },
      {
        "name": "nvme Composite SAMSUNG MZVLQ512HBLU-00B00 temp1",
        "temperature": 32.85
      },
      {
        "name": "nvme Sensor 1 SAMSUNG MZVLQ512HBLU-00B00 temp2",
        "temperature": 32.85
      }
    ]
  }
  ```

- unit: `Celsius`

#### Error

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

### Load Average

Obtain the average CPU usage within one minute, five minutes, and fifteen minutes.

> request{method=GET uri=/load_average version=HTTP/1.1}

#### Try it

```bash
curl -X GET '127.0.0.1:8000/load_average'
```

#### Success

- HTTPS status code: `200`
- Response example:

  ```json
  [{ "one": 2.22, "five": 2.34, "fifteen": 2.75 }]
  ```

- unit: `percent`

#### Error

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

### Boot Time

> request{method=GET uri=/boot_time version=HTTP/1.1}

#### Try it

```bash
curl -X GET '127.0.0.1:8000/boot_time'
```

#### Success

- HTTPS status code: `200`
- Response example:

  ```txt
  1688377060
  ```

- unit: `unix timestamp`

#### Error

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

### System Infomation aka `uname -a`

> request{method=GET uri=/sysinfo version=HTTP/1.1}

#### Try it

```bash
curl -X GET '127.0.0.1:8000/sysinfo'
```

#### Success

- HTTPS status code: `200`
- Response example:

  ```json
  [
    {
      "kernel_version": "6.2.0-20-generic",
      "os_version": "23.04",
      "long_os_version": "Linux 23.04 Ubuntu",
      "distribution_id": "ubuntu",
      "host_name": "flesh"
    }
  ]
  ```

#### Error

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

### Users

> request{method=GET uri=/users version=HTTP/1.1}

#### Try it

```bash
curl -X GET '127.0.0.1:8000/users'
```

#### Success

- HTTPS status code: `200`
- Response example:

  ```json
  [
    { "name": "root", "group": ["root"] },
    { "name": "nobody", "group": ["nogroup"] },
    { "name": "rainbow", "group": ["flesh", "sudo", "thegod", "theonetruegod"] }
  ]
  ```

#### Error

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

## Common API

### Root path

> request{method=GET uri=/ version=HTTP/1.1}

#### Response

- HTTP status code: `418`
- Response example:

  ```json
  "I'm a teapot"
  ```

### 404

> request{method=GET uri=/404 version=HTTP/1.1}

#### Response

- HTTP status code: `404`
- Response example:

  ```json
  "404"
  ```

### 500

> request{method=GET uri=/500 version=HTTP/1.1}

#### Response

- HTTP status code: `500`
- Response example:

  ```json
  "Internal Server Error"
  ```

### health

> request{method=GET uri=/health version=HTTP/1.1}

#### Response

- HTTP status code: `200`
- Response example:

  ```json
  "OK"
  ```

### support

> request{method=GET uri=/support version=HTTP/1.1}

#### Response

- HTTP status code: `200`
- Response example:

  ```json
  true
  ```

### teapot

> request{method=GET uri=/teapot version=HTTP/1.1}

#### Response

- HTTP status code: `418`
- Response example:

  ```json
  "I'm a teapot"
  ```
