import React, { FC } from 'react';
import { RouteBuilder, Routes, Route } from '@openmsupply-client/common';
import { AppRoute } from '@openmsupply-client/config';
import { DetailView, OutboundShipmentListView } from './OutboundShipment';
import {
  ListView as InboundShipmentListView,
  DetailView as InboundShipmentDetailView,
} from './InboundShipment';
import { PrescriptionListView } from './Prescriptions';

const InvoiceService: FC = () => {
  const outboundShipmentsRoute = RouteBuilder.create(
    AppRoute.OutboundShipment
  ).build();

  const outboundShipmentRoute = RouteBuilder.create(AppRoute.OutboundShipment)
    .addPart(':invoiceNumber')
    .build();

  const inboundShipmentsRoute = RouteBuilder.create(
    AppRoute.InboundShipment
  ).build();

  const inboundShipmentRoute = RouteBuilder.create(AppRoute.InboundShipment)
    .addPart(':invoiceNumber')
    .build();

  const prescriptionsRoute = RouteBuilder.create(AppRoute.Prescription).build();

  return (
    <Routes>
      <Route
        path={outboundShipmentsRoute}
        element={<OutboundShipmentListView />}
      />
      <Route path={outboundShipmentRoute} element={<DetailView />} />
      <Route
        path={inboundShipmentsRoute}
        element={<InboundShipmentListView />}
      />
      <Route
        path={inboundShipmentRoute}
        element={<InboundShipmentDetailView />}
      />
      <Route path={prescriptionsRoute} element={<PrescriptionListView />} />
    </Routes>
  );
};

export default InvoiceService;
