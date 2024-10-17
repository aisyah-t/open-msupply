import React, { FC, useEffect } from 'react';
import {
  useBreadcrumbs,
  useParams,
  useTranslation,
  useIntlUtils,
  InlineSpinner,
} from '@openmsupply-client/common';
import { usePatientVaccineCard } from '../../Vaccination/api/usePatientVaccineCard';
import { VaccinationCard } from '../../Vaccination';

export const VaccinationCardDetailView: FC = () => {
  const t = useTranslation();
  const { programEnrolmentId = '' } = useParams();
  const { setCustomBreadcrumbs } = useBreadcrumbs();
  const { getLocalisedFullName } = useIntlUtils();

  const {
    query: { data, isLoading },
  } = usePatientVaccineCard(programEnrolmentId);

  useEffect(() => {
    if (data)
      setCustomBreadcrumbs(
        {
          1: getLocalisedFullName(
            data?.patientFirstName,
            data?.patientLastName
          ),
          2: t('label.vaccination-card'),
          3: data?.programName,
        },
        [2]
      );
  }, [data]);

  return isLoading ? (
    <InlineSpinner />
  ) : (
    <VaccinationCard programEnrolmentId={programEnrolmentId} />
  );
};
