# API接口设计文档

## 1. 接口概述

本设计文档详细描述了分布式配置中心的API接口设计方案。系统同时提供gRPC和RESTful两种接口，以满足不同场景的需求。

### 1.1 接口选型

- **gRPC接口：**
  - 适用于高性能、低延迟的场景
  - 支持双向流式通信
  - 支持强类型定义
  - 支持多语言客户端

- **RESTful接口：**
  - 适用于Web应用和移动端
  - 易于调试和测试
  - 支持浏览器直接访问
  - 支持跨域请求

### 1.2 接口版本

- gRPC: v1
- RESTful: v1

## 2. 认证与授权

### 2.1 认证方式

1. **JWT认证：**
   - 使用Bearer Token
   - Token有效期：24小时
   - 支持Token刷新

2. **API Key认证：**
   - 用于服务间调用
   - 支持IP白名单
   - 支持访问频率限制

### 2.2 授权方式

1. **RBAC权限控制：**
   - 基于角色的访问控制
   - 支持细粒度的资源权限
   - 支持动态权限更新

2. **权限缓存：**
   - 缓存时间：1小时
   - 支持主动失效
   - 支持批量更新

## 3. gRPC接口设计

### 3.1 配置服务接口

```protobuf
syntax = "proto3";

package configcenter.v1;

option go_package = "github.com/your-org/config-center/api/v1;configv1";

// 配置服务
service ConfigService {
  // 获取配置
  rpc GetConfig(GetConfigRequest) returns (GetConfigResponse) {}
  
  // 设置配置
  rpc SetConfig(SetConfigRequest) returns (SetConfigResponse) {}
  
  // 删除配置
  rpc DeleteConfig(DeleteConfigRequest) returns (DeleteConfigResponse) {}
  
  // 获取配置版本
  rpc GetConfigVersion(GetConfigVersionRequest) returns (GetConfigVersionResponse) {}
  
  // 批量获取配置
  rpc BatchGetConfig(BatchGetConfigRequest) returns (BatchGetConfigResponse) {}
  
  // 批量设置配置
  rpc BatchSetConfig(BatchSetConfigRequest) returns (BatchSetConfigResponse) {}
  
  // 监听配置变更
  rpc WatchConfig(WatchConfigRequest) returns (stream WatchConfigResponse) {}
}

// 获取配置请求
message GetConfigRequest {
  string namespace = 1;
  string department = 2;
  string application = 3;
  string domain = 4;
  string environment = 5;
  string key = 6;
  bool include_metadata = 7; // 是否包含元数据
}

// 获取配置响应
message GetConfigResponse {
  string value = 1;
  ConfigMetadata metadata = 2;
}

// 配置元数据
message ConfigMetadata {
  string version = 1;
  string format = 2;
  bool is_encrypted = 3;
  string description = 4;
  string updated_by = 5;
  int64 updated_at = 6;
}

// 设置配置请求
message SetConfigRequest {
  string namespace = 1;
  string department = 2;
  string application = 3;
  string domain = 4;
  string environment = 5;
  string key = 6;
  string value = 7;
  string format = 8;
  bool is_encrypted = 9;
  string description = 10;
}

// 设置配置响应
message SetConfigResponse {
  bool success = 1;
  string version = 2;
  int64 updated_at = 3;
}

// 删除配置请求
message DeleteConfigRequest {
  string namespace = 1;
  string department = 2;
  string application = 3;
  string domain = 4;
  string environment = 5;
  string key = 6;
}

// 删除配置响应
message DeleteConfigResponse {
  bool success = 1;
}

// 获取配置版本请求
message GetConfigVersionRequest {
  string namespace = 1;
  string department = 2;
  string application = 3;
  string domain = 4;
  string environment = 5;
  string key = 6;
  string version = 7;
}

// 获取配置版本响应
message GetConfigVersionResponse {
  string value = 1;
  ConfigMetadata metadata = 2;
}

// 批量获取配置请求
message BatchGetConfigRequest {
  repeated ConfigKey keys = 1;
  bool include_metadata = 2;
}

// 配置键
message ConfigKey {
  string namespace = 1;
  string department = 2;
  string application = 3;
  string domain = 4;
  string environment = 5;
  string key = 6;
}

// 批量获取配置响应
message BatchGetConfigResponse {
  repeated ConfigValue values = 1;
}

// 配置值
message ConfigValue {
  ConfigKey key = 1;
  string value = 2;
  ConfigMetadata metadata = 3;
}

// 批量设置配置请求
message BatchSetConfigRequest {
  repeated ConfigItem items = 1;
}

// 配置项
message ConfigItem {
  ConfigKey key = 1;
  string value = 2;
  string format = 3;
  bool is_encrypted = 4;
  string description = 5;
}

// 批量设置配置响应
message BatchSetConfigResponse {
  repeated ConfigResult results = 1;
}

// 配置结果
message ConfigResult {
  ConfigKey key = 1;
  bool success = 2;
  string version = 3;
  int64 updated_at = 4;
  string error = 5;
}

// 监听配置变更请求
message WatchConfigRequest {
  repeated ConfigKey keys = 1;
  string version = 2; // 起始版本号
}

// 监听配置变更响应
message WatchConfigResponse {
  repeated ConfigChange changes = 1;
}

// 配置变更
message ConfigChange {
  ConfigKey key = 1;
  string old_value = 2;
  string new_value = 3;
  string version = 4;
  int64 updated_at = 5;
  string updated_by = 6;
}
```

### 3.2 错误码定义

```protobuf
// 错误码
enum ErrorCode {
  // 成功
  OK = 0;
  
  // 系统错误
  INTERNAL_ERROR = 1;
  SERVICE_UNAVAILABLE = 2;
  
  // 参数错误
  INVALID_ARGUMENT = 3;
  NOT_FOUND = 4;
  ALREADY_EXISTS = 5;
  
  // 权限错误
  UNAUTHORIZED = 6;
  FORBIDDEN = 7;
  
  // 业务错误
  CONFIG_NOT_FOUND = 8;
  CONFIG_ALREADY_EXISTS = 9;
  CONFIG_VERSION_NOT_FOUND = 10;
  CONFIG_FORMAT_INVALID = 11;
  CONFIG_ENCRYPTION_FAILED = 12;
  CONFIG_DECRYPTION_FAILED = 13;
}
```

## 4. RESTful接口设计

### 4.1 基础信息

- **基础路径：** `/api/v1`
- **认证方式：** Bearer Token
- **响应格式：** JSON
- **字符编码：** UTF-8

### 4.2 接口列表

#### 4.2.1 配置管理接口

1. **获取配置**

   ```plaintext
   GET /config
   ```

   **请求参数：**

   ```json
   {
     "namespace": "string",
     "department": "string",
     "application": "string",
     "domain": "string",
     "environment": "string",
     "key": "string",
     "include_metadata": "boolean"
   }
   ```

   **响应结果：**

   ```json
   {
     "value": "string",
     "metadata": {
       "version": "string",
       "format": "string",
       "is_encrypted": "boolean",
       "description": "string",
       "updated_by": "string",
       "updated_at": "integer"
     }
   }
   ```

2. **设置配置**

   ```plaintext
   POST /config
   ```

   **请求参数：**

   ```json
   {
     "namespace": "string",
     "department": "string",
     "application": "string",
     "domain": "string",
     "environment": "string",
     "key": "string",
     "value": "string",
     "format": "string",
     "is_encrypted": "boolean",
     "description": "string"
   }
   ```

   **响应结果：**

   ```json
   {
     "success": "boolean",
     "version": "string",
     "updated_at": "integer"
   }
   ```

3. **删除配置**

   ```plaintext
   DELETE /config
   ```

   **请求参数：**

   ```json
   {
     "namespace": "string",
     "department": "string",
     "application": "string",
     "domain": "string",
     "environment": "string",
     "key": "string"
   }
   ```

   **响应结果：**

   ```json
   {
     "success": "boolean"
   }
   ```

4. **获取配置版本**

   ```plaintext
   GET /config/version
   ```

   **请求参数：**

   ```json
   {
     "namespace": "string",
     "department": "string",
     "application": "string",
     "domain": "string",
     "environment": "string",
     "key": "string",
     "version": "string"
   }
   ```

   **响应结果：**

   ```json
   {
     "value": "string",
     "metadata": {
       "version": "string",
       "format": "string",
       "is_encrypted": "boolean",
       "description": "string",
       "updated_by": "string",
       "updated_at": "integer"
     }
   }
   ```

5. **批量获取配置**

   ```plaintext
   POST /config/batch
   ```

   **请求参数：**

   ```json
   {
     "keys": [
       {
         "namespace": "string",
         "department": "string",
         "application": "string",
         "domain": "string",
         "environment": "string",
         "key": "string"
       }
     ],
     "include_metadata": "boolean"
   }
   ```

   **响应结果：**

   ```json
   {
     "values": [
       {
         "key": {
           "namespace": "string",
           "department": "string",
           "application": "string",
           "domain": "string",
           "environment": "string",
           "key": "string"
         },
         "value": "string",
         "metadata": {
           "version": "string",
           "format": "string",
           "is_encrypted": "boolean",
           "description": "string",
           "updated_by": "string",
           "updated_at": "integer"
         }
       }
     ]
   }
   ```

6. **批量设置配置**

   ```plaintext
   POST /config/batch/set
   ```

   **请求参数：**

   ```json
   {
     "items": [
       {
         "key": {
           "namespace": "string",
           "department": "string",
           "application": "string",
           "domain": "string",
           "environment": "string",
           "key": "string"
         },
         "value": "string",
         "format": "string",
         "is_encrypted": "boolean",
         "description": "string"
       }
     ]
   }
   ```

   **响应结果：**

   ```json
   {
     "results": [
       {
         "key": {
           "namespace": "string",
           "department": "string",
           "application": "string",
           "domain": "string",
           "environment": "string",
           "key": "string"
         },
         "success": "boolean",
         "version": "string",
         "updated_at": "integer",
         "error": "string"
       }
     ]
   }
   ```

7. **监听配置变更**

   ```plaintext
   GET /config/watch
   ```

   **请求参数：**

   ```json
   {
     "keys": [
       {
         "namespace": "string",
         "department": "string",
         "application": "string",
         "domain": "string",
         "environment": "string",
         "key": "string"
       }
     ],
     "version": "string"
   }
   ```

   **响应结果：**

   ```json
   {
     "changes": [
       {
         "key": {
           "namespace": "string",
           "department": "string",
           "application": "string",
           "domain": "string",
           "environment": "string",
           "key": "string"
         },
         "old_value": "string",
         "new_value": "string",
         "version": "string",
         "updated_at": "integer",
         "updated_by": "string"
       }
     ]
   }
   ```

### 4.3 错误码定义

```json
{
  "error_codes": {
    "OK": 0,
    "INTERNAL_ERROR": 1,
    "SERVICE_UNAVAILABLE": 2,
    "INVALID_ARGUMENT": 3,
    "NOT_FOUND": 4,
    "ALREADY_EXISTS": 5,
    "UNAUTHORIZED": 6,
    "FORBIDDEN": 7,
    "CONFIG_NOT_FOUND": 8,
    "CONFIG_ALREADY_EXISTS": 9,
    "CONFIG_VERSION_NOT_FOUND": 10,
    "CONFIG_FORMAT_INVALID": 11,
    "CONFIG_ENCRYPTION_FAILED": 12,
    "CONFIG_DECRYPTION_FAILED": 13
  }
}
```

## 5. 接口性能指标

### 5.1 响应时间

- 95%的请求响应时间 < 100ms
- 99%的请求响应时间 < 200ms
- 99.9%的请求响应时间 < 500ms

### 5.2 吞吐量

- 单节点QPS > 1000
- 集群总QPS > 10000

### 5.3 可用性

- 服务可用性 > 99.99%
- 接口可用性 > 99.999%

## 6. 接口安全

### 6.1 传输安全

- 使用HTTPS进行传输
- 支持TLS 1.2及以上版本
- 支持HTTP/2

### 6.2 认证安全

- Token有效期控制
- Token自动刷新
- 支持多设备登录控制

### 6.3 数据安全

- 敏感数据加密传输
- 敏感数据加密存储
- 支持数据脱敏

## 7. 接口监控

### 7.1 性能监控

- 接口响应时间
- 接口调用次数
- 接口错误率
- 接口QPS

### 7.2 可用性监控

- 接口可用性
- 服务健康状态
- 依赖服务状态

### 7.3 业务监控

- 配置变更频率
- 配置访问频率
- 配置错误率
- 用户行为分析
