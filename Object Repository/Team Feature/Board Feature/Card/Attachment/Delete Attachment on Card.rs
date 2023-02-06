<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Delete Attachment on Card</name>
   <tag></tag>
   <elementGuidId>a9aa5a98-0690-481f-a920-226598c72f94</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.Token}</value>
      <webElementGuid>9887dda8-9fe7-4117-ac63-2b70f16cf389</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/api/v1/cards/${GlobalVariable.cardID}/attachments/${GlobalVariable.attachmentID}?companyId=${GlobalVariable.companyID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseURL</defaultValue>
      <description></description>
      <id>018a8bce-0f61-4e6f-a894-64ec8f671b05</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cardID</defaultValue>
      <description></description>
      <id>2f4c9c38-6153-4cf4-a177-36db63562b6d</id>
      <masked>false</masked>
      <name>cardID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.attachmentID</defaultValue>
      <description></description>
      <id>26bca624-655b-4c47-a7c8-8620091efc11</id>
      <masked>false</masked>
      <name>attachmentID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.companyID</defaultValue>
      <description></description>
      <id>032e4bfd-a3cb-4c35-bee0-c6ce4805991c</id>
      <masked>false</masked>
      <name>companyID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>f6b9ee0a-80f8-475d-ab43-093c7ceed499</id>
      <masked>false</masked>
      <name>Token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
