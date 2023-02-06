<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Board Information</name>
   <tag></tag>
   <elementGuidId>1134f65d-da45-4293-8ec7-143b3c9e99ac</elementGuidId>
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
      <webElementGuid>2ef3c073-a6a9-4582-af54-97180e5ecf4c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/v2/boards/${GlobalVariable.boardID}?limitList=10&amp;limitCard=10&amp;teamId=${GlobalVariable.teamID}&amp;companyId=${GlobalVariable.companyID}</restUrl>
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
      <id>177c9677-3d67-446e-b771-6591f512eb60</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.boardID</defaultValue>
      <description></description>
      <id>0c8ab47c-4f50-4c5b-baa3-040b28dd9b5b</id>
      <masked>false</masked>
      <name>boardID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.teamID</defaultValue>
      <description></description>
      <id>6ecccd8e-4d76-496b-98d5-d2202579557e</id>
      <masked>false</masked>
      <name>teamID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.companyID</defaultValue>
      <description></description>
      <id>891537a0-1c7f-4bd0-8655-20595d507915</id>
      <masked>false</masked>
      <name>companyID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>660e493b-0c83-435a-9742-f6c5d4cb6c64</id>
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
