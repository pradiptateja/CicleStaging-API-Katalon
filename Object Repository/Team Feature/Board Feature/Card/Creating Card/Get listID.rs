<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get listID</name>
   <tag></tag>
   <elementGuidId>118f7d7b-7f45-4be2-a766-adc614dc24c2</elementGuidId>
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
      <webElementGuid>20111fd8-c490-4c71-9f5c-4a457bf7c1cf</webElementGuid>
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
      <id>5b5b88f2-eb33-4fbd-9fe6-8957544e29cd</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.boardID</defaultValue>
      <description></description>
      <id>149fb78a-64ef-45e9-b13e-fb5577f11e81</id>
      <masked>false</masked>
      <name>boardID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.teamID</defaultValue>
      <description></description>
      <id>a0e494fd-4df2-4acf-bc29-d3f2b146f308</id>
      <masked>false</masked>
      <name>teamID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.companyID</defaultValue>
      <description></description>
      <id>73b84f64-38b1-4ebf-9b86-b13b2737a55b</id>
      <masked>false</masked>
      <name>companyID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>af47f54d-3e2d-45bd-a22e-247bd67b6554</id>
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
